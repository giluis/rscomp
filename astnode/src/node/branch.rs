use super::descriptor::Descriptor;
use super::node::{Node, NodeType};
use super::terminality::{BranchTerminality, IntoBranchTerminality};
use convert_case::{Case, Casing};
use proc_macro2::Span;
use quote::*;
use syn::spanned::Spanned;

fn and() {}

// pub struct InnerType(syn::Type);

// impl From<syn::Type> for InnerType {
//     fn from(ty: syn::Type) -> InnerType {}
// }

#[derive(Debug)]
pub struct Branch {
    pub ident: syn::Ident,
    pub type_descriptor: Descriptor,
    pub terminality: BranchTerminality,
    // pub inner_ty: InnerType,
}


impl Branch {
    pub fn to_conjunction_statement(&self) -> proc_macro2::TokenStream {
        let branch_ident = &self.ident;
        match &self.terminality {
            BranchTerminality::StatefulLeaf { source } => {
                quote! {
                    let #branch_ident =  match iter.expect_token(#source(Default::default()))?;
                }
            }
            BranchTerminality::StatelessLeaf { source } => {
                quote! {
                    let #branch_ident =  match iter.expect_token(#source)?;
                }
            }
            BranchTerminality::Reference => match &self.type_descriptor {
                Descriptor::Bare(ty) => {
                    quote! {
                        let #branch_ident = iter.parse::<#ty>()? ;
                    }
                }
                _ => unimplemented!("Optional and Repeatables have not been implemented yet"),
            },
        }
    }

    pub fn as_consumption_fn_call(&self) -> proc_macro2::TokenStream {
        let consumption_call = match (&self.terminality, &self.type_descriptor) {
            (any_terminality, Descriptor::Bare(inner_ty)) => {
                any_terminality.as_bare_consumption_fn_call(inner_ty)
            }
            (any_terminality, Descriptor::Optional(inner_ty)) => {
                any_terminality.as_optional_consumption_fn_call(inner_ty)
            }
            _ => {
                unimplemented!("Have not implemented repeatables yet")
            }
        };
        quote! { #consumption_call }
    }


    pub fn to_consumption_statement2(&self, node_type: NodeType) -> proc_macro2::TokenStream {
        let branch_ident = &self.ident;
        let inner_ty = self.type_descriptor.get_inner_ty();
        let consumption_fn_call = self.terminality.as_consumption_fn_call();

        let peekable_if_disjunct = match node_type {
            NodeType::ProductNode => quote! {},
            NodeType::SumNode => quote! {.peek()},
        };

        match node_type {
            NodeType::ProductNode => {
                quote!{
                    let #branch_ident = iter.#consumption_fn_call?;
                }
            },
            NodeType::SumNode => {
                quote!{
                    let #branch_ident = match iter
                            .consumption_fn_call{ 
                                Ok(#leaf_source(#branch_ident)) => {
                            Some(#branch_ident)
                        },
                        _ => None,
                    };
                }
            }
        }
        
        
    }

    // TODO: error handling: Different errors to simbolize lack of tokens VS wrong tokens
    // TODO: ensure iter always returns Ok(Token::A) for iter.peek_token(Token::A), or an error
    pub fn to_disjunction_statement(&self) -> proc_macro2::TokenStream {
        let branch_ident = &self.ident;
        // let snake_case_branch_ident = &self.ident.as_snake_case();
        // let (optional_descriptor, repeatable_descriptor) = self.descriptor.as_iterator_arguments();
        match &self.terminality {
            BranchTerminality::StatefulLeaf {
                source: leaf_source,
            } => {
                quote! {
                    let #branch_ident = match iter
                            // #optional_descriptor
                            // #repeatable_descriptor
                            .peek_token(#leaf_source(Default::default())) {
                        Ok(#leaf_source(#branch_ident)) => {
                            Some(#branch_ident)
                        },
                        _ => None,
                    };
                }
            }
            BranchTerminality::StatelessLeaf {
                source: leaf_source,
            } => {
                println!("{:?}", leaf_source);
                quote! {
                    let #branch_ident = match iter.peek_token(#leaf_source) {
                        Ok(#leaf_source) => {
                            Some(#leaf_source)
                        },
                        _ => None,
                    };
                }
            }
            BranchTerminality::Reference => match &self.type_descriptor {
                Descriptor::Bare(ty) => {
                    quote! {
                        let #branch_ident = match iter.attempt::<#ty>(){
                            Ok(#branch_ident) => Some(#branch_ident),
                            Err(_) => None, // do nothing for now,
                        };
                    }
                }
                _ => unimplemented!("Optional and Repeatables have not been implemented yet"),
            },
        }
    }

    pub fn to_parse_statement(&self, product_or_sum: NodeType) -> proc_macro2::TokenStream {
        match product_or_sum {
            NodeType::ProductNode => self.to_conjunction_statement(),
            NodeType::SumNode => self.to_disjunction_statement(),
        }
    }
}

impl From<&syn::Field> for Branch {
    fn from(f: &syn::Field) -> Self {
        Branch {
            ident: f.ident.clone().unwrap(),
            type_descriptor: f.ty.clone().into(),
            terminality: f.as_field_terminality(),
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
            terminality: v.as_field_terminality(),
            type_descriptor: ty.into(),
        }
    }
}

trait ChangeCase {
    fn as_snake_case(&self) -> syn::Ident;
}

impl ChangeCase for syn::Ident {
    fn as_snake_case(&self) -> syn::Ident {
        syn::Ident::new(&self.to_string().to_case(Case::Snake), self.span())
    }
}
