use astnode::AstNode;
use libcomp::token::{ Token, t};
use libcomp::iter::{TokenIter, IntoTokenIter};
use libcomp::parse::Parsable;
// use crate::common::fail;

#[derive(AstNode)]
pub enum Type {

    #[stateless_leaf( Token::KInt )]
    KInt(Token),

    // #[leaf( Token::String )]
    // KFloat(String),

    // #[leaf( Token::KInt )]
    // KChar(String),

}

// impl Parsable for Type  {
//    fn parse(iter:&mut Iter) -> Result<Type, String> {
//      
//      match iter.expect(Token::KInt)? {
//          Token::KInt(kint) => return OK(Type::KInt(kint)),
//          _ => ()
//      };
//      match iter.expect(Token::KFloat)? {
//          Token::KFloat(kfloat) => return OK(Type::KFloat(kfloat)),
//          _ => ()
//      };
//      match iter.expect(Token::KChar)? {
//          Token::KChar(kchar) => return OK(Type::KChar(kchar)),
//          _ => ()
//      };
//
//      Err("Expected Token::KInt, Token::KFloar, Token::KChar".to_string())
//    }
// }

fn main() {
    let mut iter = vec![
        t!( int ),
        t!( ident "some_function" )
    ].into_token_iter();
    let result = iter.parse::<Type>();
    match result {
        Ok(Type::KInt(Token::KInt)) => (),
        Ok(_) => panic!("There is only one variant to this enum"), // internal error: token should always be t! (int) when result is OK
        _ => panic!("Should return Ok")
    }
    // assert!(currentBefore + 1 == iter.current );
}
