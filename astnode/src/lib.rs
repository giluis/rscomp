#![allow(dead_code)]
#![allow(unused_imports)]

use syn::{parse_macro_input, DeriveInput};
use libcomp::token::Token;
use quote::*;


trait UnzippableToVec<T,P> {
    fn unzip_to_vec(self) -> (Vec<T>,Vec<P>);
}

impl <T,P,I> UnzippableToVec<T,P> for I where
I: Iterator<Item = (T,P)>
{
    fn unzip_to_vec(self) -> (Vec<T>,Vec<P>) {
        let a: (Vec<_>,Vec<_>) = self.unzip();
        return a;
    }

}


fn ty_inner_type<'a>(wrapper: &'a str, ty: &'a syn::Type) -> Option<syn::Type>{
    if let syn::Type::Path(ref p )  = ty {
        if p.path.segments.len() != 1 || p.path.segments[0].ident != wrapper {
            return None;
        }
        if let syn::PathArguments::AngleBracketed(ref inner_ty) = p.path.segments[0].arguments {
            if inner_ty.args.len() != 1 {
                return None;
            }

            let inner_ty = inner_ty.args.first().unwrap();
            if let syn::GenericArgument::Type(syn::Type::Path(t)) =  inner_ty {
                return Some(t.clone().into());
            }
        }
    }
    return None;
}

fn parse_fn(node_name: syn::Ident, fields:Vec<Field>) -> proc_macro2::TokenStream 
{
    let (field_construction, field_name) = if fields.len() == 0 {
        (vec![],vec![])
    } else {
        fields.iter()
              .map(|f| (f.to_parse_field(), f.field_name.clone()))
              .unzip_to_vec()
    };
    quote!{
        fn parse(iter: &mut TokenIter) -> Result<#node_name,String> {
            #(#field_construction);*
            Ok(#node_name {
                #(#field_name),*
            })
        }
    }

}





#[derive(Debug)]
enum FieldQualifier {
    Optional(syn::Type),//inner option type
    Repeatable(syn::Type),// inner vec type
    Node(syn::Type), // f.field_type
    Leaf{
        from_token: syn::Path, // Token::Variant
        value_ty: syn::Type // whatever is inside the Token Variant
    }, // get the token
}

impl FieldQualifier {
    fn from_field(_f: &syn::Field) -> Self {
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

#[derive(Debug)]
struct Field {
    field_name: syn::Ident,
    field_type: syn::Type,
    qualifier: FieldQualifier
}

fn error(f: &syn::Field, msg: &'static str)->syn::Error {
    syn::Error::new(f.ident.clone().unwrap().span(),msg)
}

impl Field {
    fn new(f: &syn::Field)->Result<Self, syn::Error>{
        let ident =  match &f.ident {
            Some(i) => i.clone(),
            None => unimplemented!("Fields with no identifiers have not been implemented yet"),
        };

        Ok ( Field {
            field_name: ident,
            field_type: f.ty.clone(),
            qualifier: FieldQualifier::from_field(f)
        } )
    }

    fn to_parse_field(&self) -> proc_macro2::TokenStream {
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
}


// fn repeatable(f: &syn::Field ) -> Option<syn::Ident> {
//     for attr in f.attrs.iter(){
//         if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "builder" {
//             let next = attr.clone().tokens.into_iter().next();
//             if let Some(proc_macro2::TokenTree::Group(g)) = next{
//                 let mut giter = g.stream().into_iter();
//                 let _each = giter.next();
//                 let _equalsign = giter.next();
//                 let arg = match giter.next().unwrap(){
//                     proc_macro2::TokenTree::Literal(l) => l,
//                     tt => panic!("Expected string, found {}", tt),
//                 };
//                 match syn::Lit::new(arg) {
//                     syn::Lit::Str(s) => {
//                         return Some(syn::Ident::new( &s.value(), s.span() ));
//                     },
//                     lit => panic!("Expected string, found {:?}", lit),
//                 };
//
//             }
//         }
//     };
//     return None;
// }
//


#[proc_macro_derive(AstNode, attributes(token))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let node_name = &ast.ident;
    let fields = match get_fields(&ast){
        Ok(f) => f,
        Err(_) => unimplemented!("Not sure what to do here")
    };  

    let parsefn = parse_fn(node_name.clone(), fields); 
    quote!{ 
        impl Parsable for #node_name {
            #parsefn
        }

    }.into()
}

fn get_fields<'a>(ast: &DeriveInput) -> Result<Vec<Field>, syn::Error> {
    let raw_fields = match &ast.data {
            syn::Data::Struct(syn::DataStruct {
                fields: syn::Fields::Named(syn::FieldsNamed {
                    named: fields,
                    ..
                }),
                ..
            }) => fields,
            _ => unimplemented!("What to do when fields are not named")
        };
    raw_fields.iter().map(|f|Field::new(f)).collect()
}

fn is_field_optional(f:&syn::Field ) -> bool {
    let op = ty_inner_type("Option",&f.ty); 
    op.is_some()
}
