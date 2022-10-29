use super::descriptor::Descriptor;
use super::node::{Node, NodeType};
use super::terminality::{BranchTerminality, IntoFieldTerminality};
use quote::*;
use syn::spanned::Spanned;
use convert_case::Case;

#[derive(Debug)]
pub struct Branch {
    pub ident: syn::Ident,
    pub descriptor: Descriptor,
    pub terminality: BranchTerminality,
}

impl Branch {
    pub fn to_conjunction_statement(&self) -> proc_macro2::TokenStream {
        let branch_ident = &self.ident;
        match &self.terminality {
            BranchTerminality::StatefulLeaf {
                source,
            } => {
                quote! {
                    let #branch_ident =  match iter.get_next() {
                        Some(#source(#branch_ident)) => #branch_ident,
                        None => return Err(format!("No more tokens")),
                        _ => return Err(format!("Expected a diffefent token")),
                    };
                }
            }, 
            BranchTerminality::StatelessLeaf { source } => {
                quote! {
                    let #branch_ident =  match iter.get_next() {
                        Some(#source) => #source,
                        None => return Err(format!("No more tokens")),
                        _ => return Err(format!("Expected a diffefent token")),
                    };
                }

            }
            BranchTerminality::Reference => match &self.descriptor {
                Descriptor::Bare(ty) => {
                    quote! {
                        let #branch_ident = iter.parse::<#ty>()? ;
                    }
                }
                _ => unimplemented!("Optional and Repeatables have not been implemented yet"),
            },
        }
    }

                        // TODO: error handling: Different errors to simbolize lack of tokens VS wrong tokens
                        // TODO: ensure iter always returns Ok(Token::A) for iter.peek_token(Token::A), or an error
    pub fn to_disjunction_statement(&self, node_name: &syn::Ident) -> proc_macro2::TokenStream {
        let branch_ident = &self.ident;
        match &self.terminality {
            BranchTerminality::StatefulLeaf {
                source: leaf_source,
            } => {
                quote! {
                    match iter.peek_token(#leaf_source(Default::default())) {
                        Ok(#leaf_source(leaf_source_content)) => {
                            return Ok(#node_name::#branch_ident(leaf_source_content))
                        },
                        _ => (), 
                        // TODO: "leaf_source_content" should be generated as lower case
                    };
                }
            }, 
            BranchTerminality::StatelessLeaf { source: source_token } => {
                quote! {
                    match iter.peek_token(#source_token) {
                        Ok(#source_token) => {
                            return Ok(#node_name::#branch_ident(#source_token))
                        },
                        _ => (),
                        // TODO: error handling: Different errors to simbolize lack of tokens VS wrong tokens
                        // TODO: ensure iter always returns Ok(Token::A) for iter.peek_token(Token::A), or an error
                    };
                }

            }
            BranchTerminality::Reference => match &self.descriptor {
                Descriptor::Bare(ty) => {
                    quote! {
                        match iter.attempt::<#ty>(){
                            Ok(#branch_ident) => return Ok(#node_name::#branch_ident(#branch_ident)),
                            Err(_) => (), // do nothing for now,

                        };
                        // TODO: "branch_ident" should be generated as lower case
                    }
                }
                _ => unimplemented!("Optional and Repeatables have not been implemented yet"),
            },
        }
    }

    pub fn to_parse_statement(
        &self,
        product_or_sum: NodeType,
        node_name: &syn::Ident,
    ) -> proc_macro2::TokenStream {
        match product_or_sum {
            NodeType::ProductNode => self.to_conjunction_statement(),
            NodeType::SumNode => self.to_disjunction_statement(node_name),
        }
    }
}

impl From<&syn::Field> for Branch {
    fn from(f: &syn::Field) -> Self {
        Branch {
            ident: f.ident.clone().unwrap(),
            descriptor: f.ty.clone().into(),
            terminality: f.into_field_terminality(),
        }
    }
}

fn error<T>(e: &T, msg: &str) -> syn::Error
where
    T: Spanned,
{
    syn::Error::new(e.span(), msg)
}

trait LeafSourceExtractable {
    fn extract_leaf_source_from_atribute(self) -> Result<syn::TypePath, syn::Error>;
}

impl From<&syn::Variant> for Branch {
    fn from(v: &syn::Variant) -> Branch {
        dbg!(v);
        let ty = match &v.fields {
            syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }) => match unnamed.first() {
                Some(a) => &a.ty,
                None => unimplemented!("what to do when enum Variants are field less"),
            },
            _ => unimplemented!("Can enums have named fields"),
        };

        Branch {
            ident: v.ident.clone(),
            terminality: v.into_field_terminality(),
            descriptor: ty.into(),
        }
    }
}
