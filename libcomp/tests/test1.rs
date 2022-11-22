

#[cfg(test)]
mod tests {
    use crate::token::Token;
    use crate::lexer::lex;
    use crate::parse::Parsable;
    use crate::t; 

    #[derive(PartialEq, Debug)]
    struct TestStruct {
        var_type: Token,
        var_name: String,
        equals_sign: String,
        value: u32,
    }

    impl Parsable for TestStruct {
        fn parse(iter: &mut TokenIter) -> Result<TestStruct, String>{

            let var_type = iter.expect(t!( int ))?;

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
    fn disjunct_expect() {
        let mut iter = TokenIter::new(vec![
            t!(litint 32)
        ]);

        let result = iter.disjunct_expect(t!(litint 32));
        
        let expected = DisjunctResultWrapper::<Token,String>(Ok(t!(litint 32)));

        assert!(expected == result);

    }

    #[test]
    fn peek(){
        let expected_variable = "variable1";
        let expected_value = 3;
        let mut iter = TokenIter::new(vec![
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
            var_type: t!(int),
            var_name: expected_variable.to_string(),
            equals_sign: t!( = def ).to_string(),
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
        let mut iter = TokenIter::new(vec![
                t!( int ),
                Token::Identifier(expected_var_name.to_string()),
                t!( = ),
                Token::LiteralInt(expected_value),
                t!( ; )

        ]);
        let expected_struct = TestStruct {
            var_type: t!( int ),
            var_name: expected_var_name.to_string(),
            equals_sign: t!( = def ).to_string(),
            value: expected_value,
        };
        let r_struct:TestStruct = iter.parse().unwrap();
        assert_eq!(r_struct, expected_struct);
    }



    #[test]
    fn peek_token(){
        let mut iter = TokenIter::new(vec![
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
        let mut iter = TokenIter::new(vec![
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
        let mut iter = TokenIter::new(lex("int variable = 3;".to_string()).unwrap());

        iter.increment();
        iter.increment();
        assert_eq!(iter.current, 2);
    }

    #[test]
    fn test_expect_empty_tokenlist(){
        let mut iter = TokenIter::new(lex("".to_string()).unwrap());


        let result = iter.expect(t!(l_paren));
        assert!(result.is_err());
    }


    #[test]
    fn test_expect(){
        let mut iter = TokenIter::new(lex("(1,2,3,4,5)".to_string()).unwrap());


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
        let iter = TokenIter::new(lex("(1,2,3,4,5)".to_string()).unwrap());
        assert_eq!(iter.current, 0);
        assert_eq!(iter.tokens.len(),11);
        assert_eq!(iter.size, 11);
        assert_eq!(iter.stack.len(), 0);
    }

    #[test]
    fn test_new_empty(){
        let iter = TokenIter::new(vec![]);
        assert_eq!(iter.current, 0);
        assert_eq!(iter.tokens.len(),0);
        assert_eq!(iter.size,0);
        assert_eq!(iter.stack.len(), 0);
    }
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

