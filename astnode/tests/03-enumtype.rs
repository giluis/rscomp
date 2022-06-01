// Resources:
// 
//   - The Quote crate for putting together output from a macro:
//     https://github.com/dtolnay/quote
//
//   - Joining together the type name + "Builder" to make the builder's name:
//     https://docs.rs/syn/1.0/syn/struct.Ident.html

use astnode::AstNode;


#[derive(AstNode)]
pub enum Type {

    #[token( Token::KInt )]
    KInt(String),

    #[token( Token::String )]
    KFloat(String),

    #[token( Token::KInt )]
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
    let iter = Iter::new(vec![
        t!( int )
        t!( ident "some_function" )
    ]);
    let currentBefore = iter.current;
    let result = iter.parse::<Type>();
    match result {
        Type::KInt(_) => (),
        _ => assert!(false)
    }
    assert!(currentBefore + 1, iter.current );
}
