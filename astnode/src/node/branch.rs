use super::descriptor::Descriptor;
use super::node::{Node, NodeType};
use super::terminality::{BranchTerminality, IntoFieldTerminality};
use quote::*;
use syn::spanned::Spanned;

#[derive(Debug)]
pub struct Branch {
    pub ident: syn::Ident,
    pub desc: Descriptor,
    pub terminality: BranchTerminality,
}

impl Branch {
    pub fn to_conjunction_statement(&self) -> proc_macro2::TokenStream {
        let branch_ident = &self.ident;
        match &self.terminality {
            BranchTerminality::Leaf {
                source: leaf_source,
            } => {
                quote! {
                    let #branch_ident =  match iter.get_next() {
                        Some(#leaf_source(#branch_ident)) => #branch_ident,
                        None => return Err(format!("No more tokens")),
                        _ => return Err(format!("Expected a diffefent token")),
                    };
                }
            }
            BranchTerminality::Reference => match &self.desc {
                Descriptor::Bare(ty) => {
                    quote! {
                        let #branch_ident = iter.parse::<#ty>()? ;
                    }
                }
                _ => unimplemented!("Optional and Repeatables have not been implemented yet"),
            },
        }
    }

    pub fn to_disjunction_statement(&self, node_name: &syn::Ident) -> proc_macro2::TokenStream {
        let branch_ident = &self.ident;
        match &self.terminality {
            BranchTerminality::Leaf {
                source: leaf_source,
            } => {
                quote! {
                    match iter.peek_token(#leaf_source) {
                        Ok(#leaf_source(leaf_source_content)) => {
                            return Ok(#node_name::#branch_ident(leaf_source_content))
                        },
                        Ok(_) => {
                            return Err("Iotarnal error: ok result on Iter.peek_token(Token::SomeToken) should always return Token::SomeToken".to_string())
                        },
                        Err(_) => (), // do nothing for now,
                        // TODO: Different errors to simbolize lack of tokens VS wrong tokens
                    };
                }
            }
            BranchTerminality::Reference => match &self.desc {
                Descriptor::Bare(ty) => {
                    quote! {
                        match iter.attempt::<#ty>(){
                            Ok(r) => return #node_name::#branch_ident(r),
                            Err(_) => (), // do nothing for now,

                        };
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
            desc: f.ty.clone().into(),
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
            desc: ty.into(),
        }
    }
}
