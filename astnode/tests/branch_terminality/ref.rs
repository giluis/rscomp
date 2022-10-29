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

    // #[stateful_leaf( Token::Identifier )]
    // ident: String,

    // #[leaf( Token::Assign )]
    // equals_sign: String,


    // #[leaf( Token::LiteralInt )]
    // value: u32

    // omitted expression (fearing recursion)
}

// impl Parsable for AssignStatement  {
//    fn parse(iter:&mut Iter) -> Result<AssignStatement, String> {
//      let ty = iter.parse::<Type>()?;
//
//      Ok(AssignStatement {
//          ty,
//      })
//      
//    }
// }

#[derive(AstNode, PartialEq)]
pub struct Type {
    #[stateless_leaf(Token::KInt)]
    int: Token,
}



// impl Parsable for Type  {
//    fn parse(iter:&mut Iter) -> Result<Type, String> {
//      let int = match iter.expect_token(Token::KInt) ? {
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
    ].into_token_iter()
     .parse::<AssignStatement>();

    let expected = AssignStatement::new(Type::new(Token::KInt));
    assert!(Ok(expected) == result);

}
