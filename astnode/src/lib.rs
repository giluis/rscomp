#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(let_chains)]
// #![feature(adt_const_params)]
// #![warn(missing_docs)]

mod util;
mod node_lib;

use node_lib::node::{Node, NodeType};
use node_lib::descriptor::Descriptor;
use node_lib::branch::Branch;
use syn::{parse_macro_input, DeriveInput};
use libcomp::token::Token;
use quote::*;

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

