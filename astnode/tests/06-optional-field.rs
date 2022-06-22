use astnode::AstNode;


#[derive(AstNode)]
pub struct Parameter {
    ty: Type,
    ident: Identifier,

    #[token(Token::SemiColon)]
    optional_semi: Option<String>,
} 
//
// impl Parsable for Parameter  {
//    fn parse(iter:&mut Iter) -> Result<Parameter, String> {
//      
//      let ty = match iter.parse::<Type>?;
//      let ident = match iter.parse::<Identifier>?;
//      let optional_semi = match iter.expect(Token::SemiColon) {
//          Ok(s) => match s => {
//              Token::SemiColon(s) => Some(s),
//              _ => panic!("Internal Error: expected semicolon"),
//          }, 
//          Err(_) => None
//      }
//      return OK(Paremeter {
//          ty,
//          ident,
//          optional_semi
//      })
//    }
// }
//


#[derive(AstNode)]
pub struct Identifier { 
    #[token(Identifier)]
    value: String,
}


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
        t!( int ),
        t!( ident "some_parameter" ),
        t! ( ; ),
    ]);
    let currentBefore = iter.current;
    let result = iter.parse::<Parameter>();
    match result {
        Ok(f) => {
            assert!(matches!(f.ty, Type::KInt(_)));
            assert!(f.ident.value, "some_paremeter".to_string());
            assert!(f.optional_semi.is_some())
        },
        _ => panic!("Expected Ok result1"),
    }

    assert!(currentBefore + 1, iter.current );

    let iter = Iter::new(vec![
        t!( int ),
        t!( ident "some_parameter" ),
    ]);
    let currentBefore = iter.current;
    let result = iter.parse::<Parameter>();
    match result {
        Ok(f) => {
            assert!(matches!(f.ty, Type::KInt(_)));
            assert!(f.ident.value, "some_paremeter".to_string());
            assert!(f.optional_semi.is_none())
        },
        _ => panic!("Expected Ok result2"),
    }
    assert!(currentBefore + 1, iter.current );
}


