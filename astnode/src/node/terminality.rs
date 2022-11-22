use quote::*;

#[derive(Debug)]
pub enum BranchTerminality {
    Reference,
    StatefulLeaf { source: syn::TypePath },
    StatelessLeaf { source: syn::TypePath },
}

impl BranchTerminality {
    pub fn as_bare_consumption_fn_call(&self, inner_ty: &syn::Type)-> proc_macro2::TokenStream {
        match self {
            Self::StatefulLeaf { source}
             =>  { quote! {
                    .expect_token(#source(Default::default())) 
                        
            } },
            BranchTerminality::StatelessLeaf  {
                source
            } =>  quote! {
                    .expect_token(#source) 
            },
            BranchTerminality::Reference  => {
                quote!{
                    .parse::<#inner_ty>()
                }
                
            } 
        }
    }

    pub fn as_optional_consumption_fn_call(&self, inner_ty: &syn::Type) -> proc_macro2::TokenStream {
        match self {
            Self::StatefulLeaf { source}
            =>  { quote! {
                    .expect_optional_token(#source(Default::default())) 
                        
            } },
            Self::StatelessLeaf  {
                source
            } =>  quote! {
                    .expect_optional_token(#source) 
            },
            Self::Reference  => {
                quote!{
                    .parse_optional::<#inner_ty>()
                }
                
            } 
        }

    }
}


pub trait IntoBranchTerminality {
    fn as_field_terminality<'a>(&'a self) -> BranchTerminality
    where
        Self: HasAttributes<'a> + syn::spanned::Spanned + Sized,
    {
        match self
            .get_attrs()
            .find(|attr| /* attr.path.segments.len() == 1 && */ attr.path.segments[0].ident == "stateful_leaf" || attr.path.segments[0].ident == "stateless_leaf" )
        {
            None => BranchTerminality::Reference,
            Some(attr) => {        
                let source = attr.parse_args::<syn::TypePath>()
                         .expect("Could not extract leaf source from attribute");
                if attr.path.segments[0].ident == "stateful_leaf" {
                    BranchTerminality::StatefulLeaf { source }
                } else {
                    BranchTerminality::StatelessLeaf { source }
                }
            }
        }
    }
}

impl<'a> IntoBranchTerminality for &syn::Variant {}
impl<'a> IntoBranchTerminality for &syn::Field {}

pub trait HasAttributes<'a> {
    fn get_attrs(&self) -> std::slice::Iter<'a, syn::Attribute>;
}

impl<'a> HasAttributes<'a> for &'a syn::Field {
    fn get_attrs(&self) -> std::slice::Iter<'a, syn::Attribute> {
        self.attrs.iter()
    }
}

impl<'a> HasAttributes<'a> for &'a syn::Variant {
    fn get_attrs(&self) -> std::slice::Iter<'a, syn::Attribute> {
        self.attrs.iter()
    }
}
