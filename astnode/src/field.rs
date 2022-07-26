use quote::*;
use crate::util::ty_inner_type;

#[derive(Debug)]
pub struct Field {
    pub ident: syn::Ident,
    pub full_type: syn::Type,
    pub inner_ty: syn::Type,
    category: FieldCategory,
    qualifier: FieldQualifier,
}

#[derive(Debug)]
pub enum FieldCategory {
    Node,
    Leaf {
        leaf_source: syn::Path
    }
} 





/*
 *             #[leaf(Token::Identifier)]
 *             ident: Option<String>,
 * qualifier <---|       |-----> ty
 * is_leaf: true
 *
 *               Vec<Function>,
 * qualifier <---|       |-----> ty
 * is_leaf: false
 *
 * */
#[derive(Debug)]
pub struct FieldType {
    qualifier: FieldQualifier, // Optional, Repeatable, others
    ty: syn::Type, // actual type
    is_leaf: bool, 
}

impl From<&syn::Field> for Field {
    fn from(f: &syn::Field)->Self{
        let ident =  match &f.ident {
            Some(i) => i.clone(),
            None => unimplemented!("Fields with no identifiers have not been implemented yet"),
        };

        Field {
            ident,
            full_type: f.ty.clone(),
            inner_ty: Self::extract_inner_ty(&f.ty),
            qualifier: FieldQualifier::from_field(f),
            is_leaf: true,
        }
    }
}


impl Field {

    pub fn to_parse_field(&self) -> proc_macro2::TokenStream {
        let field_name = &self.field_name;
        match &self.qualifier {
            FieldQualifier::Leaf {from_token, ..} => {
                quote!{
                    let #field_name =  match iter.get_next() {
                        Some(#from_token(#field_name)) => #field_name,
                        None => return Err(format!("No more tokens")),
                        _ => return Err(format!("Expected a diffefent token")),
                     };
                }
            },
            _ => quote!{}/* unimplemented!("Not yet implemented all qualifiers") */
        }

    }
    pub fn extract_inner_ty(ty: &syn::Type) -> syn::Type {


    }
}




#[derive(Debug)]
pub enum FieldQualifier {
    Optional(syn::Type),//inner option type
    Repeatable(syn::Type),// inner vec type
    Node(syn::Type), // f.field_type
    Leaf{
        from_token: syn::Path, // Token::Variant
        value_ty: syn::Type // whatever is inside the Token Variant
    }, // get the token
}

impl FieldQualifier {
    pub fn from_field(_f: &syn::Field) -> Self {
             // Self::Node(f.ty.clone())
        match syn::parse::<syn::Path>(quote!{Token::Identifier}.into()) {
                Ok(from_token) => match syn::parse::<syn::Type>(quote!{String}.into()) { 
                    Ok(value_ty) => Self::Leaf{from_token,value_ty},
                    _=> panic!("Could not parse value_ty")
                },
                _=> panic!("Could not parse value_ty")
        }
        
        // if let Some(ty) = extract_repeatable(f) {
        //      Self::Repeatable(ty)
        // } else if let Some(ty) = extract_optional(f) {
        //      Self::Optional(ty)
        // } else if let Some((from_token, value_ty)) = extract_leaf(f) {
        //      Self::Leaf {from_token, value_ty}
        // } else {

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

