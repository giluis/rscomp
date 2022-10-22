// Resources:
//
//   - The Quote crate for putting together output from a macro:
//     https://github.com/dtolnay/quote
//
//   - Joining together the type name + "Builder" to make the builder's name:
//     https://docs.rs/syn/1.0/syn/struct.Ident.html

use astnode::AstNode;
use libcomp::token::{ Token, t};
use libcomp::iter::{TokenIter, IntoTokenIter};
use libcomp::parse::Parsable;


#[derive(AstNode, PartialEq)]
pub struct AssignStatement {
    ty: Type,

    #[leaf( Token::Identifier )]
    ident: String,

    #[leaf( Token::Assign )]
    equals_sign: String,


    #[leaf( Token::LiteralInt )]
    value: u32

    // omitted expression (fearing recursion)
}

// impl Parsable for Identifier  {
//    fn parse(iter:&mut Iter) -> Result<Identifier, String> {
//      let ty = iter.parse::<Type>()?;
//
//      let ident = match iter.expect(Token::Identifier) ? {
//          Token::Identifier(ident) => ident,
//          _ => panic!("Internal error: Ok result for iter.expect should always yield token of the same kind as input "),
//      }
//
//      let equals_sign = match iter.expect(Token::Assign) ? {
//          Token::Assign(equals_sign) => equals_sign,
//          _ => panic!("Internal error: Ok result for iter.expect should always yield token of the same kind as input ")),
//      }
//
//      Ok(AssignStatement {
//          ty,
//          ident,
//          equals_sign
//      })
//      
//    }
// }

#[derive(AstNode, PartialEq)]
pub struct Type {
    #[leaf(Token::KInt)]
    int: String,
}



// impl Parsable for Type  {
//    fn parse(iter:&mut Iter) -> Result<Type, String> {
//      let int = match iter.expect(Token::KInt) ? {
//          Token::KInt(int) => int,
//          _ => panic!("Internal error: Ok result for iter.expect should always yield token of the same kind as input "),
//      }
//
//      Ok(Type {
//          int
//      })
//    }
// }
//


fn main() {
    let result = vec![
            t!( int ),
            t!( ident "var1" ),
            t!( = ),
            t!( litint 5 )
    ].into_token_iter()
     .parse::<AssignStatement>();
    let expected = AssignStatement::new(Type::new("int".to_string()),"var1".to_string(), "=".to_string(), 5);
    assert!(Ok(expected) == result);


    let result = vec![
            t!( ident "var1" ),
            t!( = ),
            t!( litint 5 )
    ].into_token_iter()
     .parse::<AssignStatement>();
    let expected = AssignStatement::new(Type::new("int".to_string()),"var1".to_string(), "=".to_string(), 5);
    assert!(Ok(expected) != result);


}
