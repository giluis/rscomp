use super::descriptor::Descriptor;
use super::node::{Node, NodeType};
use super::terminality::{BranchTerminality, IntoBranchTerminality};
use convert_case::{Case, Casing};
use proc_macro2::Span;
use quote::*;
use syn::spanned::Spanned;


#[derive(Debug)]
pub struct Branch {
    pub ident: syn::Ident,
    pub terminality: BranchTerminality,
    pub ty: syn::Type,
    // pub inner_ty: InnerType,
    // pub full_type: syn::Type,
}

impl Branch {
    pub fn to_consumption_statement(&self, node_name: &syn::Ident, node_type: &NodeType) -> proc_macro2::TokenStream {
        let (consumption_fn_call, assignment) = match node_type {
            NodeType::SumNode => (
                
                    self.terminality.as_disjunct_fn_call(node_name, &self.ident) ,
                    format_ident!("{}_err", &self.ident.as_snake_case()),
            ),
            // TODO: remove this clone()
            NodeType::ProductNode => (self.terminality.as_conjunct_fn_call(), self.ident.clone()),
        };
        quote! {let #assignment = iter.#consumption_fn_call?;}
    }
}

impl From<&syn::Field> for Branch {
    fn from(f: &syn::Field) -> Self {
        Branch {
            ident: f.ident.clone().unwrap(),
            terminality: f.as_branch_terminality(),
            ty: f.ty.clone(),
        }
    }
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
            ty: ty.clone(),
            terminality: v.as_branch_terminality(),
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
