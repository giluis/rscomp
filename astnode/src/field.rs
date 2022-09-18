use crate::util::ty_inner_type;
use quote::*;
use syn::spanned::Spanned;

#[derive(Debug)]
pub struct Field {
    pub name: syn::Ident,
    pub ty: FieldType,
    pub terminality: FieldTerminality,
}

impl Field {
    pub fn to_parse_field(&self) -> proc_macro2::TokenStream {
        let field_ident = &self.name;
        match &self.terminality {
            FieldTerminality::Leaf {
                source: leaf_source,
            } => {
                quote! {
                    let #field_ident =  match iter.get_next() {
                        Some(#leaf_source(#field_ident)) => #field_ident,
                        None => return Err(format!("No more tokens")),
                        _ => return Err(format!("Expected a diffefent token")),
                     };
                }
            }
            FieldTerminality::NodeRef => match &self.ty {
                FieldType::Bare(ty) => {
                    quote! {
                        let #field_ident = iter.parse::<#ty>()? ;
                    }
                }
                _ => unimplemented!("Optional and Repeatables have not been implemented yet"),
            },
        }
    }
}

impl From<&syn::Field> for Field {
    fn from(f: &syn::Field) -> Self {
        Field {
            name: f.ident.clone().unwrap(),
            ty: (&f.ty).into(),
            terminality: f.into(),
        }
    }
}

#[derive(Debug)]
pub enum FieldTerminality {
    NodeRef,
    Leaf { source: syn::TypePath },
}

fn error<T>(e: T, msg: &str) -> syn::Error
where
    T: Spanned,
{
    syn::Error::new(e.span(), msg)
}

trait LeafSourceExtractable {
    fn extract_leaf_source_from_atribute(self) -> Result<syn::TypePath, syn::Error>;
}

// impl LeafSourceExtractable for &syn::Attribute {
//     fn extract_leaf_source_from_atribute(self) -> Result<syn::TypePath, syn::Error> {
//         let next = self.tokens.into_iter().next();
//         if let Some(proc_macro2::TokenTree::Group(g)) = next {
//             syn::parse(g.stream().into())
//         } else {
//             Err(error(
//                 self,
//                 "Could not extract source from attribute on field",
//             ))
//         }
//     }
// }

fn extract_leaf_source_from_atribute(attr: &syn::Attribute) -> Result<syn::TypePath, syn::Error> {
    let next = attr.clone().tokens.into_iter().next();
    if let Some(proc_macro2::TokenTree::Group(g)) = next {
        syn::parse(g.stream().into())
    } else {
        Err(error(
            attr,
            "Could not extract source from attribute on field",
        ))
    }
}
impl FieldTerminality {
    fn is_node(f: &syn::Field) -> bool{
        f.attrs.iter().count() > 0
    }
    fn extract_leaf_source(f: &syn::Field) -> Result<syn::TypePath, syn::Error> {
        f.attrs
            .iter()
            .filter(|attr| {
                attr.path.segments.len() == 1 && attr.path.segments[0].ident == "leaf"
            })
            .nth(0)
            .ok_or(error(f, "Field is not leaf"))
            .and_then(extract_leaf_source_from_atribute)
    }
}

impl From<&syn::Field> for FieldTerminality {
    fn from(f: &syn::Field) -> Self {

        match Self::extract_leaf_source(f) {
            _ if FieldTerminality::is_node(f) => panic!("Something"), 
            Ok(source) => Self::Leaf { source },
            Err(_) => panic!("could not parse leaf"),
        }
    }
}

#[derive(Debug)]
pub enum FieldType {
    Optional(&'static syn::Type),   //inner option type
    Repeatable(&'static syn::Type), // inner vec type
    Bare(&'static syn::Type),
}

impl From<&syn::Type> for FieldType {
    fn from(ty: &syn::Type) -> Self {
        match ty.extract_if_optional()
        .or_else(||ty.extract_if_repeatable()){
            Some(t) => t,
            None => FieldType::Bare(&ty.clone())
        }
    }
}

trait ModifierExtractable<'a> {
    fn extract_if_repeatable(self) -> Option<FieldType>;

    fn extract_if_optional(self) -> Option<FieldType>;
}

impl <'a> ModifierExtractable<'a> for &syn::Type {
    fn extract_if_repeatable(self) -> Option<FieldType> {
        ty_inner_type("Vec", self.clone())
        .and_then(|t|Some(FieldType::Repeatable(t)))
        .or(None)
        
    }

    fn extract_if_optional(self) -> Option<FieldType> {
        ty_inner_type("Option", self.clone())
        .and_then(|t|Some(FieldType::Optional(t)))
        .or(None)
    }
}
