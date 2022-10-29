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

#[derive(AstNode, PartialEq)]
pub struct Arg {
    #[stateful_leaf(Token::Identifier)]
    ident: String,

    #[stateful_leaf(Token::LiteralInt)]
    semi: u32,
}

// impl Parsable for Identifier  {
//    fn parse(iter:&mut Iter) -> Result<Identifier, String> {
//      let ident = match iter.get_next() {
//          Some(Token::Identifier(ident)) => ident,
//          Some(other_token) => return Err(format!("Expected Token::Identifier, but got {}", other_token))
//          _ => return Err(format!("No more tokens",_)),
//      }
//      let semi = match iter.get_next() {
//          Some(Token::semi(semi)) => semi,
//          Some(other_token) => return Err(format!("Expected Token::semi, but got {}", other_token))
//          _ => return Err(format!("No more tokens",_)),
//      }
//      Ok(Arg{ident, ty, semi})
//    }
// }

fn main() {

    let mut iter = TokenIter::new(vec![
        t!( ident "some_ident" ),
        t!( litint 32 )
    ]);

    let result = Arg::parse(&mut iter);
    let expected = Arg::new("some_ident".to_string(), 32);
    
    assert!(result == Ok(expected));
}
