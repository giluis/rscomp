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
pub struct Function {
    ident: Identifier,
    #[collection(
        del=(LParen, RParen),
        sep=Comma
    )]
    args: Vec<Arg>,
}


#[derive(AstNode)]
pub struct Arg {
    ty: Type,
    ident: Identifier,
}

#[derive(AstNode)]
pub struct Identifier {
    #[token(Token::Identifier)]
    value: String,
}

#[derive(AstNode)]
pub struct LParen {
    #[token(Token::LParen)]
    value: String,
}

// impl Parsable for LParen  {
//    fn parse(iter:&mut Iter) -> Result<Type, String> {
//       let value = match iter.expect(Token::LParen)? {
//          Token::LParen(value) => value
//
//       }
//       Ok(LParen{
//          value
//       });
//    }
// }



#[derive(AstNode)]
pub struct RParen {
    #[token(Token::RParen)]
    value: String,
}
//
// impl Parsable for RParen  {
//    fn parse(iter:&mut Iter) -> Result<Type, String> {
//       let value = match iter.expect(Token::RParen)? {
//          Token::RParen(value) => value
//
//       }
//       Ok(RParen{
//          value
//       });
//    }
// }



#[derive(AstNode)]
pub struct Comma {
    #[token(Token::Comma)]
    value: String,
}

// impl Parsable for Comma  {
//    fn parse(iter:&mut Iter) -> Result<Type, String> {
//       let value = match iter.expect(Token::Comma)? {
//          Token::Comma(value) => value
//
//       }
//       Ok(Comma{
//          value
//       });
//    }
// }


// impl Parsable for Function  {
//    fn parse(iter:&mut Iter) -> Result<Type, String> {
//          let ty = iter.collection::<Arg>()
//                       .delimiter::<?;
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
        t!( ident "some_function" ),
        t!( lparen ),
        t!( int ), t!( ident "arg_0" ), t!( , )
        t!( int ), t!( ident "arg_1" ), t!( , )
        t!( int ), t!( ident "arg_2" ),
        t!( rparen ),
    ]);
    let currentBefore = iter.current;
    let result = iter.parse::<Function>();
    match result {
        Ok(f) => {
            assert!(f.ident, "some_function");
            assert!(f.args[0].ident, "arg_0");
            assert!(f.args[1].ident, "arg_1");
            assert!(f.args[2].ident, "arg_2");
            assert!(f.args.len(), 3);
        },
        _ => assert!(false),
    }
    assert!(currentBefore + 1, iter.current );
}

