
use super::branch::Branch;
use crate::util::UnzippableToVec;
use crate::Descriptor;
use quote::*;
use std::slice::Iter;
use syn::DataStruct;
use syn::DeriveInput;
use syn::TypePath;



#[derive(PartialEq,Eq)]
enum Stage {
    Lanched, 
    Grounded
}

struct Rocket<const Stage:Stage>; 

#[derive(Debug)]
pub enum NodeType {
    ProductNode,
    SumNode,
}

#[derive(Debug)]
pub struct Node {
    pub node_type: NodeType,
    pub ident: syn::Ident,
    branches: Vec<Branch>,
}

impl Node {
    fn is_empty(self) -> bool {
        self.branches.len() == 0
    }

    fn parse_fn_body(self) -> proc_macro2::TokenStream {
        let node_name = self.ident;
        quote! {
            fn parse(iter: &mut TokenIter) -> Result<#node_name,String> {
            }
        }
    }
    pub fn to_consumption_statements<'a>(
        &'a self,
    ) -> (Vec<proc_macro2::TokenStream>, Vec<&'a syn::Ident>) {
        self.branches
            .iter()
            .map(|b| {
                (
                    match self.node_type {
                        NodeType::ProductNode => b.to_conjunction_statement(),
                        NodeType::SumNode => b.to_disjunction_statement(&self.ident),
                    },
                    &b.ident,
                )
            })
            .unzip_to_vec()
    }


    pub fn disjunct_node_constructor(&self) -> proc_macro2::TokenStream {
        let branches = self.branches.into_iter().map(|b|&b.ident).collect::<Vec<&syn::Ident>>();
        let node_ident = &self.ident;
        match self.node_type {
            NodeType::ProductNode => {
                // TODO: Best way to handle this.
                panic!("Can only be called on sumNodes. This is an internal error, please report")
            },
            NodeType::SumNode => {
                quote!{
                    Ok(#node_ident:: {
                        #(#branches,)*
                    })
                }
            }
        }
    }

    pub fn to_parse_fn(&self) -> proc_macro2::TokenStream {
        let node_name = &self.ident;
        let (consumption_statements, branch_idents) = self.to_consumption_statements();
        let fn_body = match self.node_type {
            NodeType::SumNode => {
                    quote! {
                    #(#consumption_statements)*
                    return Err("could not parse any of the variants for this sum node".to_string());
                }
            }
            NodeType::ProductNode => {
                quote! {
                    #(#consumption_statements)*
                    return Ok(#node_name{
                        #(#branch_idents),*
                    })
                }
            }
        };
        quote! {
            fn parse(iter: &mut TokenIter) -> Result<#node_name,String> {
                #fn_body
            }
        }
    }


    pub fn to_newfn(&self) -> proc_macro2::TokenStream {
        let node_ident = &self.ident;
        let (args, instantiation_fields) = self.branches.iter().map(|f|{
            let fty = match &f.type_descriptor {
            Descriptor::Optional(t) => quote!{Option<#t>},
            Descriptor::Repeatable(t) => quote!{Vec<#t>},
            Descriptor::Bare(t) => quote!{#t},
            };
            let fident = &f.ident;
            (quote!{
                #fident: #fty
            },quote!{#fident})
        }).unzip_to_vec();
        quote!{
            fn new(#(#args),*) -> Self {
                #node_ident {
                #(#instantiation_fields),*
                }
            }
        }
    }
}

impl From<syn::DeriveInput> for Node {
    fn from(derive_input: syn::DeriveInput) -> Self {
        let (branches, node_type) = match derive_input.data {
            syn::Data::Struct(data_struct) => {
                (data_struct.fields.into_branches(), NodeType::ProductNode)
            }
            syn::Data::Enum(data_enum) => (data_enum.variants.into_branches(), NodeType::SumNode),
            _ => unimplemented!("Nodes from unions are not implemented"),
        };
        Node {
            branches,
            node_type,
            ident: derive_input.ident,
        }
    }
}

trait IntoBranches {
    fn into_branches(self) -> Vec<Branch>;
}

impl IntoBranches for syn::Fields {
    fn into_branches(self) -> Vec<Branch> {
        match self {
            syn::Fields::Named(syn::FieldsNamed {
                named: fields_named,
                ..
            }) => fields_named
                .pairs()
                .map(|f| f.into_value().into())
                .collect(),
            _ => unimplemented!(
                "Unimplemented: Unnamed syn::fields. Should this be allowed by the API?"
            ),
        }
    }
}

impl IntoBranches for syn::punctuated::Punctuated<syn::Variant, syn::token::Comma> {
    fn into_branches(self) -> Vec<Branch> {
        self.iter().map(|v| v.into()).collect()
    }
}
