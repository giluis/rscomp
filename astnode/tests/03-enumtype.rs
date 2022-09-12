use astnode::AstNode;
use libcomp::token::{ Token, t};
use libcomp::iter::{TokenIter, IntoTokenIter};
use libcomp::parse::Parsable;

#[derive(AstNode)]
pub enum Type {

    #[leaf( Token::KInt )]
    KInt(String),

    #[leaf( Token::String )]
    KFloat(String),

    #[leaf( Token::KInt )]
    KChar(String),

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
    let iter = TokenIter::new(vec![
        t!( int ),
        t!( ident "some_function" )
    ]);
    let currentBefore = iter.current;
    let result = iter.parse::<Type>();
    match result {
        Type::KInt(_) => (),
        _ => assert!(false)
    }
    assert!(currentBefore + 1 == iter.current );
}
