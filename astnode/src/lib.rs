#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(let_chains)]
// #![warn(missing_docs)]

mod util;
mod node;

use node::node::{Node, NodeType};
use node::descriptor::Descriptor;
use syn::{parse_macro_input, DeriveInput};
use libcomp::token::Token;
use quote::*;
use crate::node::branch::Branch;

use util::{ty_inner_type, UnzippableToVec};


#[proc_macro_derive(AstNode, attributes(stateless_leaf, stateful_leaf))]
pub fn parse_consumer(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    // dbg!(ast.clone());
    let node:Node = ast.into();
    let newfn = match node.node_type  {
        NodeType::ProductNode =>  node.to_newfn(),
        NodeType::SumNode => quote!(),
    };
    let parsefn =   node.to_parse_fn();


    // println!("{:?}",node);
    // println!("{}",parsefn);
    let node_ident = &node.ident;
    
    quote!{ 
        impl Parsable for #node_ident {
            #parsefn
        }

        impl #node_ident {
            #newfn
        }

    }.into()
}

fn newfn(node_name: &syn::Ident, fields: &Vec<Branch>) -> proc_macro2::TokenStream {

    let (args, instantiation_fields) = fields.iter().map(|f|{
        let fty = match &f.descriptor {
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
            #node_name {
               #(#instantiation_fields),*
            }
        }
    }
}

fn get_fields<'a>(ast: &DeriveInput) -> Vec<Branch>{
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

