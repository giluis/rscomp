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
use libcomp::iter::Iter;

#[derive(AstNode)]
pub struct Identifier {
    #[ast( from = Token::Identifier )]
    ident: String,
}

// impl Parsable for Identifier  {
//    fn parse(iter:&mut Iter) -> Result<Identifier, String> {
//      match iter.get_next() {
//          Token::Identifier(ident) => Ok(Identifier { ident }),
//          _ => Err(format!("Expected Token::Identifier, but got {}",_)),
//      }
//    }
// }

fn main() {
    let iter = Iter::new(vec![
        t!( ident "some_ident" )
    ]);
    let result = Identifier::parse(iter);
    assert!(result.ident == "some_ident".to_string());

}
