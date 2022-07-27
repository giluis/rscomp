#![allow(dead_code)]
#![allow(unused_imports)]

use syn::{parse_macro_input, DeriveInput};
use libcomp::token::Token;
use quote::*;
mod field;
mod util;
use field::{Field, FieldType};

use util::{ty_inner_type, UnzippableToVec};


#[proc_macro_derive(AstNode, attributes(leaf))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let node_name = &ast.ident;
    let fields = get_fields(&ast);
    let newfn = newfn(&node_name, &fields);
    let parsefn = parse_fn(node_name.clone(), fields); 
    quote!{ 
        impl Parsable for #node_name {
            #parsefn
        }

        impl #node_name {
            #newfn
        }

    }.into()
}

fn newfn(node_name: &syn::Ident, fields: &Vec<Field>) -> proc_macro2::TokenStream {

    let (args, instantiation_fields) = fields.iter().map(|f|{
        let fty = match &f.ty {
           FieldType::Optional(t) => quote!{Option<#t>},
           FieldType::Repeatable(t) => quote!{Vec<#t>},
           FieldType::Bare(t) => quote!{#t},
        };
        let fident = &f.ident;
        (quote!{
            #fident: #fty
        },quote!{#fident})
    }).unzip_to_vec();
    quote!{
        fn new(#(#args),*) {
            #node_name {
               #(#instantiation_fields),*
            }
        }
    }
}

fn get_fields<'a>(ast: &DeriveInput) -> Vec<Field>{
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
    raw_fields.iter().map(|f|f.into()).collect()
}



fn parse_fn(node_name: syn::Ident, fields:Vec<Field>) -> proc_macro2::TokenStream 
{
    let (field_assignment_to_parse_result, field_name) = if fields.len() == 0 {
        (vec![],vec![])
    } else {
        fields.iter()
              .map(|f| (f.to_parse_field(), f.ident.clone()))
              .unzip_to_vec()
    };
    quote!{
        fn parse(iter: &mut TokenIter) -> Result<#node_name,String> {
            #(#field_assignment_to_parse_result);*
            Ok(#node_name {
                #(#field_name),*
            })
        }
    }
}


fn error(f: &syn::Field, msg: &'static str)->syn::Error {
    syn::Error::new(f.ident.clone().unwrap().span(),msg)
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

