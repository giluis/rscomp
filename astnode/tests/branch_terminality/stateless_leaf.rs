use astnode::AstNode;
use libcomp::token::{ Token, t};
use libcomp::iter::TokenIter;
use libcomp::parse::Parsable;

// pub enum Type {
//     #[stateless_leaf(Token::KInt)]
//     KInt,
//     #[stateless_leaf(Token::KFloat)]
//     KFloat,
// }

// impl Parsable for Type  {
//    fn parse(iter:&mut Iter) -> Result<Type, String> {
//      let kint = match iter.expect() {
//          Some(Token::KInt) => ident,
//          Some(other_token) => Err(format!("Expected Token::Identifier, but got {}", other_token))
//          _ => Err(format!("Expected Token::Identifier, but got {}",_)),
//      }
//      Ok(Identifier{ident})
//    }
// }

#[derive(AstNode, PartialEq)]
pub struct KInt {
    #[stateless_leaf(Token::KInt)]
    kint: Token,
}

// impl Parsable for KInt  {
//    fn parse(iter:&mut Iter) -> Result<KInt, String> {
//      let kint = match iter.get_next() {
//          Some(Token::Identifier) => Token::Identifier,
//          Some(other_token) => Err(format!("Expected Token::Identifier, but got {}", other_token))
//          _ => Err(format!("Expected Token::Identifier, but got {}",_)),
//      }
//      Ok(Identifier{ident})
//    }
// }

fn main() {

    let mut iter = TokenIter::new(vec![
        t!( int )
    ]);
    let result = KInt::parse(&mut iter);
    
    assert!(result.unwrap().kint == Token::KInt);


    let mut ty = TokenIter::new(vec![
        t!( int )
    ]);
    
}
