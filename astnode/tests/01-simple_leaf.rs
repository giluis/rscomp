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
pub struct Identifier {
    #[leaf(Token::Identifier)]
    ident: String,
}

#[derive(AstNode, PartialEq)]
pub struct Arg {
    #[leaf(Token::Identifier)]
    ident: String,

    #[leaf(Token::Identifier)]
    ty: String,
    
    #[leaf(Token::Comma)]
    comma: String,
}

// impl Parsable for Identifier  {
//    fn parse(iter:&mut Iter) -> Result<Identifier, String> {
//      let ident = match iter.get_next() {
//          Some(Token::Identifier(ident)) => ident,
//          Some(other_token) => Err(format!("Expected Token::Identifier, but got {}", other_token))
//          _ => Err(format!("Expected Token::Identifier, but got {}",_)),
//      }
//      Ok(Identifier{ident})
//    }
// }

fn main() {

    let mut iter = TokenIter::new(vec![
        t!( ident "some_ident" )
    ]);
    let result = Identifier::parse(&mut iter);
    
    assert!(result.unwrap().ident == "some_ident".to_string());

    let mut iter = TokenIter::new(vec![
        t!( ident "int" )
        t!( ident "arg1" )
        t!( , )
    ]);
    let result = Arg::parse(&mut iter);
    let expected = Arg::new()
    
    assert!(result.unwrap().ident == "some_ident".to_string());
}
