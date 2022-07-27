use quote::*;
use crate::util::ty_inner_type;

#[derive(Debug)]
pub struct Field {
    pub ident: syn::Ident,
    pub ty: FieldType,
    category: FieldCategory,
}

#[derive(Debug)]
pub enum FieldCategory {
    Node,
    Leaf {
        source: syn::Path
    }
} 

impl From<&syn::Field> for FieldCategory {
    fn from (f: &syn::Field) -> Self {
        match Self::extract_leaf_source(f) {
            Some(Ok(source)) => Self::Leaf {source},
            Some(Err(_)) => panic!("Leaf source could not be parsed"),
            None => Self::Node
        }
    }
}

impl FieldCategory {
    fn extract_leaf_source(f: &syn::Field) -> Option<Result<syn::Path, syn::Error>> {
        for attr in f.attrs.iter(){
            if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "leaf" {
                let next = attr.clone().tokens.into_iter().next();
                if let Some(proc_macro2::TokenTree::Group(g)) = next{
                     return Some(syn::parse(g.stream().into()));
                }
            }
        }
        None
    }
}



impl From<&syn::Field> for Field {
    fn from(f: &syn::Field)->Self{
        Field {
            ident: f.ident.clone().unwrap(),
            ty: (&f.ty).into(),
            category: f.into(),
        }
    }
}


impl Field {

    pub fn to_parse_field(&self) -> proc_macro2::TokenStream {
        let field_ident = &self.ident;
        match &self.category {
            FieldCategory::Leaf {source: leaf_source} => {
                quote!{
                    let #field_ident =  match iter.get_next() {
                        Some(#leaf_source(#field_ident)) => #field_ident,
                        None => return Err(format!("No more tokens")),
                        _ => return Err(format!("Expected a diffefent token")),
                     };
                }
            },
            _ => unimplemented!{"Node category has not been implemented"}
        }

    }
}




#[derive(Debug)]
pub enum FieldType {
    Optional(syn::Type),//inner option type
    Repeatable(syn::Type),// inner vec type
    Bare(syn::Type)
}

impl From<&syn::Type> for FieldType {
    fn from(ty: &syn::Type) -> Self {
        FieldType::Bare(ty.clone())
    }
}

fn extract_repeatable(f: &syn::Field) -> Option<syn::Type> {
    ty_inner_type("Vec",&f.ty)
}

fn extract_optional(f: &syn::Field) -> Option<syn::Type> {
    ty_inner_type("Option",&f.ty)
}

fn extract_leaf(f: &syn::Field) -> Option<(syn::Path, syn::Type)> {
    for attr in f.attrs.iter(){
        if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "token" {
            let next = attr.clone().tokens.into_iter().next();
            if let Some(proc_macro2::TokenTree::Group(_g)) = next{
                // let mut giter = g.stream().into_iter();
                // let _each = giter.next();
                // let _equalsign = giter.next();
                // let arg = match giter.next().unwrap(){
                //     proc_macro2::TokenTree::Literal(l) => l,
                //     tt => panic!("Expected string, found {}", tt),
                // };
                // match syn::Lit::new(arg) {
                //     syn::Lit::Str(s) => {
                //         return Some(syn::Ident::new( &s.value(), s.span() ));
                //     },
                //     lit => panic!("Expected string, found {:?}", lit),
                // };

            }
        }
    };
    return None;
}

    // pub fn from_field(_f: &syn::Field) -> Self {
    //          // Self::Node(f.ty.clone())
    //     match syn::parse::<syn::Path>(quote!{Token::Identifier}.into()) {
    //             Ok(from_token) => match syn::parse::<syn::Type>(quote!{String}.into()) { 
    //                 Ok(value_ty) => Self::Leaf{from_token,value_ty},
    //                 _=> panic!("Could not parse value_ty")
    //             },
    //             _=> panic!("Could not parse value_ty")
    //     }
    //     
    //     // if let Some(ty) = extract_repeatable(f) {
    //     //      Self::Repeatable(ty)
    //     // } else if let Some(ty) = extract_optional(f) {
    //     //      Self::Optional(ty)
    //     // } else if let Some((from_token, value_ty)) = extract_leaf(f) {
    //     //      Self::Leaf {from_token, value_ty}
    //     // } else {
    //
    // }
