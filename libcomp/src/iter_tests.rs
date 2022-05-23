#[cfg(test)]
mod iter_tests {
    use super::super::*;
    use crate::token::{ Token, KINT_DEFAULT_STRING,ASSIGN_DEFAULT_STRING, LiteralIntValue,IdentifierValue, LiteralStringValue };
    use crate::lexer::lex;
    use crate::t; 

    #[derive(PartialEq, Debug)]
    struct TestStruct {
        var_type: String,
        var_name: String,
        equals_sign: String,
        value: u32,
    }

    impl Parsable for TestStruct {
        fn parse(iter: &mut Iter) -> Result<TestStruct, String>{

            let var_type = match iter.expect(t!( int ))? {
                Token::KInt(int) => int.to_string(),
                _ =>panic!("Internal error, should be ident_str"), 
            };

            let ident_str = match iter.expect(t!( ident ))? {
                Token::Identifier(ident_str)=> ident_str,
                _ => panic!("Internal error, should be ident_str"),
            };

            let _equals_sign = iter.expect(t!( = )).unwrap();
            let value = match iter.expect(t!( litint )).unwrap() {
                Token::LiteralInt(value) => value.clone(),
                _ => panic!("Internal error: should be lit int")
            };
            Ok(TestStruct { var_type,  var_name: ident_str, equals_sign: "=".to_string(), value })
        }
    }


    #[test]
    fn peek(){
        let expected_variable = "variable1";
        let expected_value = 3;
        let mut iter = Iter::new(vec![
                t!( int ),
                t!( int ),
                Token::Identifier(expected_variable.to_string()),
                t!( = ),
                Token::LiteralInt(expected_value),
                t!( ; )

        ]);
        // ignore initial value
        let current_iter = iter.increment();

        let expected_struct = TestStruct {
            var_type: KINT_DEFAULT_STRING.to_string(),
            var_name: expected_variable.to_string(),
            equals_sign: ASSIGN_DEFAULT_STRING.to_string(),
            value: expected_value,
        };
        let r_struct:TestStruct = iter.peek().unwrap();
        assert_eq!(expected_struct, r_struct);

        let r_struct:TestStruct = iter.peek().unwrap();
        assert_eq!(expected_struct, r_struct);

        // no change to current pointer
        assert_eq!(iter.current, current_iter);

        iter.increment();

        // fails, since current pointer is not at beginning of struct
        assert!(iter.peek::<TestStruct>().is_err())
    }


    #[test]
    fn parse(){
        let expected_var_name = "variable1";
        let expected_value = 3;
        let mut iter = Iter::new(vec![
                t!( int ),
                Token::Identifier(expected_var_name.to_string()),
                t!( = ),
                Token::LiteralInt(expected_value),
                t!( ; )

        ]);
        let expected_struct = TestStruct {
            var_type: KINT_DEFAULT_STRING.to_string(),
            var_name: expected_var_name.to_string(),
            equals_sign: ASSIGN_DEFAULT_STRING.to_string(),
            value: expected_value,
        };
        let r_struct:TestStruct = iter.parse().unwrap();
        assert_eq!(r_struct, expected_struct);
    }



    #[test]
    fn peek_token(){
        let mut iter = Iter::new(vec![
                t!( int ),
                Token::Identifier("variable".to_string()),
                t!( = ),
                Token::LiteralInt(2),
                t!( ; )

        ]);
        let rint = iter.peek_token(t!( int ));
        let rident = iter.peek_token(t!( ident  ));
        assert!(rint.is_ok());
        assert!(rident.is_err());
        assert_eq!(iter.current, 0);
        iter.increment();

        let rident = iter.peek_token(t!( ident  ));
        assert!(rident.unwrap() == Token::Identifier("variable".to_string()));
        assert_eq!(iter.current, 1);
    }

    #[test]
    fn test_push_pop(){
        let mut iter = Iter::new(vec![
                t!( int ),
                Token::Identifier("variable".to_string()),
                t!( = ),
                Token::LiteralInt(2),
                t!( ; )

        ]);
        iter.push();
        assert_eq!(iter.stack, vec![ 0 ]);
        let r = iter.expect(t!( int ));
        assert!(r.is_ok());
        assert_eq!(iter.current, 1);
        iter.pop();
        assert_eq!(iter.current, 0);
    }

    #[test]
    fn test_increment(){
        let mut iter = Iter::new(lex("int variable = 3;".to_string()).unwrap());

        iter.increment();
        iter.increment();
        assert_eq!(iter.current, 2);
    }

    #[test]
    fn test_expect_empty_tokenlist(){
        let mut iter = Iter::new(lex("".to_string()).unwrap());


        let result = iter.expect(t!(l_paren));
        assert!(result.is_err());
    }


    #[test]
    fn test_expect(){
        let mut iter = Iter::new(lex("(1,2,3,4,5)".to_string()).unwrap());


        let lparen_r = iter.expect(t!(l_paren));
        assert!(lparen_r.unwrap() == t!(l_paren));

        let rparen_r = iter.expect(t!( r_paren ));
        assert!(rparen_r.is_err());

        let comma_r = iter.expect(t!( , ));
        assert_eq!(comma_r.unwrap(),t!( , ));

        let litint_r  = iter.expect(t!( litint ));
        assert!(litint_r.unwrap() == Token::LiteralInt(2));
    }


    #[test]
    fn test_new(){
        let iter = Iter::new(lex("(1,2,3,4,5)".to_string()).unwrap());
        assert_eq!(iter.current, 0);
        assert_eq!(iter.tokens.len(),11);
        assert_eq!(iter.size, 11);
        assert_eq!(iter.stack.len(), 0);
    }

    #[test]
    fn test_new_empty(){
        let iter = Iter::new(vec![]);
        assert_eq!(iter.current, 0);
        assert_eq!(iter.tokens.len(),0);
        assert_eq!(iter.size,0);
        assert_eq!(iter.stack.len(), 0);
    }
}
