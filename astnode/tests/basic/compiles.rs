// Resources:
//
//   - The Syn crate for parsing procedural macro input:
//     https://github.com/dtolnay/syn
//
//   - The DeriveInput syntax tree which represents input of a derive macro:
//     https://docs.rs/syn/1.0/syn/struct.DeriveInput.html
//
//   - An example of a derive macro implemented using Syn:
//     https://github.com/dtolnay/syn/tree/master/examples/heapsize


use astnode::AstNode;
use libcomp::token::{ Token, t};
use libcomp::iter::TokenIter;
use libcomp::parse::Parsable;

#[derive(AstNode)]
pub struct Identifier {}

// impl Parsable for Identifier  {
//    fn parse(iter:&mut Iter) -> Result<Identifier, String> {
//      Ok(Identifier{})
//    }
// }

fn main() {

    // println!("Compiled");
    let mut _iter = TokenIter::new(vec![
        t!( ident "some_ident" )
    ]);
    let result = Identifier::parse(&mut _iter);
    assert!(result.is_ok())

}
