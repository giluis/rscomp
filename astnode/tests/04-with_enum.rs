// Resources:
// 
//   - The Quote crate for putting together output from a macro:
//     https://github.com/dtolnay/quote
//
//   - Joining together the type name + "Builder" to make the builder's name:
//     https://docs.rs/syn/1.0/syn/struct.Ident.html
// Generate a `build` method to go from builder to original struct.
//
// This method should require that every one of the fields has been explicitly
// set; it should return an error if a field is missing. The precise error type
// is not important. Consider using Box<dyn Error>, which you can construct
// using the impl From<String> for Box<dyn Error>.
//
//     impl CommandBuilder {
//         pub fn build(&mut self) -> Result<Command, Box<dyn Error>> {
//             ...
//         }
//     }


use astnode::AstNode;

#[derive(AstNode)]
pub struct VarDecaration {
    ty: Type,

    #[token( Token::Identifier )]
    ident: String,
}

// impl Parsable for Type  {
//    fn parse(iter:&mut Iter) -> Result<Type, String> {
//          let ty = iter.parse::<Type>()?;
//          let ident = iter.expect(Token::Identifier)? {
//              Token::Identifier(ident) => ident, 
//              _ => panic!("Error, internal consistency"),
//          }
//          return Ok(VarDeclaration{
//              ty,
//              ident
//          })
//    }
// 
// }




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
//

fn main() {
    let iter = Iter::new(vec![
        t!( int )
        t!( ident "some_function" )
    ]);
    let currentBefore = iter.current;
    let result = iter.parse::<VarDeclaration>();
    match result {
        Ok(VarDeclaration{
            ty: Type::KInt(_),
            ident: Token::Identifier(ident), 
        }) => (),
        _ => assert!(false),
    }
    assert!(currentBefore + 1, iter.current );
}
