

#[cfg(test)]
mod collection_builder_tests {
    use super::super::*;
    use crate::{ t, collection, fail };
    use crate::tests_common::*;
    use crate::token::Token;

    #[test]
    fn parse_nodel_nosep(){
        let (varname1 ,varname2,varname3) = ("var1" ,"var2" ,"var3");
        let mut iter = TokenIter::new(vec![
                t!( int ),
                t!( ident varname1),
                t!( int ),
                t!( ident varname2),
                t!( int ),
                t!( ident varname3),

        ]);
        let r = iter.collection::<TestStruct>().parse();
        let expected_collection = collection![
            TestStruct::from_string(varname1),
            TestStruct::from_string(varname1),
            TestStruct::from_string(varname1),
        ];

        let result_collection = match r {
            Ok(col) => col,
            Err(_) => fail!("Expected successsful collection parsed")
        };
        assert!(expected_collection == result_collection);
    }


    #[test]
    fn parse_del_nosep(){
        let (varname1, varname2) =("var1", "var2");

        let mut iter = TokenIter::new(vec![
                t!( l_paren ),
                t!( int ),
                t!( ident varname1 ),
                t!( int ),
                t!( ident varname2 ),
                t!( r_paren ),

        ]);

        let r = iter.collection::<TestStruct>()
                                .delimiter::<LParen,RParen>()
                                .parse();

        let expected_collection: Collection<TestStruct, LParen, RParen> = collection!(           
            [
                TestStruct::from_string(varname1),
                TestStruct::from_string(varname2),
            ],
            del=(LParen{}, RParen{})
        );

        match r {
            Ok(result_collection) => assert!(expected_collection == result_collection),
            Err(_) => fail!("Expected successsful collection parsed")
        };
      
    }

    #[test]
    fn parse_nodel_sep(){
        let (varname1 ,varname2,varname3) = ("var1" ,"var2" ,"var2");
        let mut iter = TokenIter::new(vec![
                t!( int ),
                t!( ident varname1 ),
                t!( , ),
                t!( int ),
                t!( ident varname2 ),
                t!( , ),
                t!( int ),
                t!( ident varname3 ),
        ]);

        let r = iter.collection::<TestStruct>()
                                .separator::<Comma>()
                                .parse();
        let expected_collection = collection!(
            [
              TestStruct::from_string(varname1),
              TestStruct::from_string(varname2),
              TestStruct::from_string(varname3),
            ],
            sep=[Comma{},Comma{},],
        );

        match r {
            Ok(result_collection) => assert!(expected_collection == result_collection),
            Err(_) => fail!("Expected successsful collection parsed")
        };
    }

    #[test]
    fn parse_nodel_sep_fail(){
        let (varname1 ,varname2,varname3) = ("var1" ,"var2" ,"var2");
        let mut iter = TokenIter::new(vec![
                t!( int ),
                t!( ident varname1 ),
                t!( , ),
                t!( int ),
                t!( ident varname2 ),
                t!( , ),
                t!( int ),
                t!( ident varname3 ),
        ]);

        let r = iter.collection::<TestStruct>()
                                // missing separator
                                // .separator::<Comma>()
                                .parse();


        match r {
            // assert resulting collection is only the first element
            // separator was not specified
            Ok(result_collection) => assert!(result_collection.len() == 1), 
            Err(_) => fail!("Expected success, since a single Test struct is parsed")
        };
    }


    #[test]
    fn parse_del_sep(){
        let (varname1, varname2, varname3)  =("var1", "var2", "var3");
        let mut iter = TokenIter::new(vec![
                t!( l_curly ),
                t!( int ),
                t!( ident varname1 ),
                t!( , ),
                t!( int ),
                t!( ident varname2 ),
                t!( , ),
                t!( int ),
                t!( ident varname3 ),
                t!( r_curly ),
        ]);

        let r = iter.collection::<TestStruct>()
                                .delimiter::<LCurly,RCurly>()
                                .separator::<Comma>()
                                .parse();

        let expected_collection = collection!{
            [
                TestStruct::from_string(varname1),
                TestStruct::from_string(varname2),
                TestStruct::from_string(varname3),
            ],
            del=(LCurly{}, RCurly{}),
            sep=[Comma{}, Comma{},],
        };

        match r {
            Ok(result_collection) => assert!(expected_collection == result_collection),
            Err(_) => fail!("Expected successsful collection parsed")
        };
    }

    #[test]
    fn parse_del_sep_fail(){
        let (varname1 ,varname2,varname3) = ("var1" ,"var2" ,"var2");
        let mut iter = TokenIter::new(vec![
                t!( l_curly ),
                t!( int ),
                t!( ident varname1 ),
                t!( , ),
                t!( int ),
                t!( ident varname2 ),
                t!( , ),
                t!( int ),
                t!( ident varname3 ),
                t!( r_paren ),
        ]);

        let r = iter.collection::<TestStruct>()
                                .delimiter::<LCurly,RCurly>()
                                .separator::<Comma>()
                                .parse();

        // should fail, since r_paren is no r_curly
        assert!(r.is_err());
    }

    #[test]
    fn parse_successful_min_len(){
        let (varname1 ,varname2,varname3) = ("var1" ,"var2" ,"var2");
        let mut iter = TokenIter::new(vec![
                t!( l_bracket ),
                t!( int ),
                t!( ident varname1 ),
                t!( , ),
                t!( int ),
                t!( ident varname2 ),
                t!( , ),
                t!( int ),
                t!( ident varname3 ),
                t!( r_bracket ),
        ]);

        let r = iter.collection::<TestStruct>()
                                .delimiter::<LBracket,RBracket>()
                                .separator::<Comma>()
                                .min_len(2)
                                .parse();

        let expected_collection = collection!(
            [
                TestStruct::from_string(varname1),
                TestStruct::from_string(varname2),
                TestStruct::from_string(varname3),
            ],
            del=(LBracket{}, RBracket{}),
            sep=[Comma{}, Comma{},],
        );

        match r {
            Ok(result_collection) => assert!(expected_collection == result_collection),
            Err(_) => fail!("Expected successful collection comparison")
        }
    }

    #[test]
    fn parse_failed_min_len(){
        let varname1 = "var1";
        let mut iter = TokenIter::new(vec![
                t!( l_bracket ),
                t!( int ),
                t!( ident varname1 ),
                t!( r_bracket ),
        ]);

        let min_len = 2;
        let r = iter.collection::<TestStruct>()
                                .delimiter::<LBracket,RBracket>()
                                .min_len(min_len)
                                .parse();
        match r {
            Ok(_)=> fail!("Expected failed result, since minlen is greater than actual len"), //TODO fail test
            Err(_) => () ,
        };
    }


    #[test]
    fn parse_failed_min_len4(){
        let varname1 = "var1";
        let mut iter = TokenIter::new(vec![
                t!( l_bracket ),
                t!( int ),
                t!( ident varname1 ),
                t!( , ),
                t!( int ),
                t!( ident varname1 ),
                t!( , ),
                t!( int ),
                t!( ident varname1 ),
                t!( r_bracket ),
        ]);

        let min_len = 4;
        let r = iter.collection::<TestStruct>()
                                .delimiter::<LBracket,RBracket>()
                                .separator::<Comma>()
                                .min_len(min_len)
                                .parse();
        match r {
            Ok(_)=> fail!("Expected failed result, since minlen is greater than actual len"),
            Err(_) => () ,
        };
    }

    // inspired by double object destructuring
    #[test]
    fn parse_doubledel_singlesep(){
        let (varname1 ,varname2,varname3) = ("var1" ,"var2" ,"var2");
        let mut iter = TokenIter::new(vec![
                t!( l_bracket ),
                t!( l_bracket ),
                t!( int ),
                t!( ident varname1 ),
                t!( , ),
                t!( int ),
                t!( ident varname2 ),
                t!( , ),
                t!( int ),
                t!( ident varname3 ),
                t!( r_bracket ),
                t!( r_bracket ),
        ]);

        let r = iter.collection::<TestStruct>()
                                .delimiter::<DoubleLBracket,DoubleRBracket>()
                                .separator::<Comma>()
                                .min_len(2)
                                .parse();

        let expected_collection = collection![
            [
                TestStruct::from_string(varname1),
                TestStruct::from_string(varname2),
                TestStruct::from_string(varname3),
            ],
            del=(DoubleLBracket{}, DoubleRBracket{}),
            sep=[Comma{}, Comma {},],

        ];

        match r {
            Ok(result_collection) => assert!(expected_collection == result_collection),
            Err(_) => fail!("Expected successful collection comparison")
        }

    }

    // inspired by lua string composition
    #[test]
    fn parse_nodel_doublesep(){
        let (varname1 ,varname2,varname3) = ("var1" ,"var2" ,"var2");
        let mut iter = TokenIter::new(vec![
                t!( int ),
                t!( ident varname1 ),
                t!( , ),
                t!( , ),
                t!( int ),
                t!( ident varname2 ),
                t!( , ),
                t!( , ),
                t!( int ),
                t!( ident varname3 ),
        ]);

        let r = iter.collection::<TestStruct>()
                                .separator::<DoubleComma>()
                                .min_len(2)
                                .parse();

        let expected_collection = collection![
            [
                TestStruct::from_string(varname1),
                TestStruct::from_string(varname2),
                TestStruct::from_string(varname3),
            ],
            sep=[DoubleComma{}, DoubleComma{},],
        ];

        match r {
            Ok(result_collection) => assert!(expected_collection == result_collection),
            Err(_) => fail!("Expected successful collection comparison")
        }

    }

    // inspired by javascript arrays

    #[test]
    fn parse_nodel_complex_sep(){
        let (varname1 ,varname2,varname3) = ("var1" ,"var2" ,"var2");
        let mut iter = TokenIter::new(vec![
                t!( int ),
                t!( ident varname1 ),
                t!( , ),
                t!( , ),
                t!( , ),
                t!( int ),
                t!( ident varname2 ),
                t!( , ),
                t!( int ),
                t!( ident varname3 ),
        ]);
        let r = iter.collection::<TestStruct>()
                                .separator::<AnyNumberOfCommas>()
                                .min_len(2) //casually testing min len with complex sep
                                .parse();

        let expected_collection = collection![
            [
                TestStruct::from_string(varname1),
                TestStruct::from_string(varname2),
                TestStruct::from_string(varname3),
            ],
            sep=[AnyNumberOfCommas{}, AnyNumberOfCommas{},],
        ];

        match r {
            Ok(result_collection) => assert!(expected_collection == result_collection),
            Err(_) => fail!("Expected successful collection comparison")
        }
    }

    #[test]
    fn parse_doubledel_complex_sep(){
        let (varname1 ,varname2,varname3) = ("var1" ,"var2" ,"var2");
        let mut iter = TokenIter::new(vec![
                t!( l_curly ),
                t!( l_curly ),
                t!( int ),
                t!( ident varname1 ),
                t!( , ),
                t!( , ),
                t!( , ),
                t!( int ),
                t!( ident varname2 ),
                t!( , ),
                t!( int ),
                t!( ident varname3 ),
                t!( r_curly ),
                t!( r_curly ),
        ]);
        let r = iter.collection::<TestStruct>()
                                .delimiter::<DoubleLCurly,DoubleRCurly>()
                                .separator::<AnyNumberOfCommas>()
                                .min_len(2) //casually testing min len with complex sep
                                .parse();

        let expected_collection = collection![
            [
                TestStruct::from_string(varname1),
                TestStruct::from_string(varname2),
                TestStruct::from_string(varname3),
            ],
            del=(DoubleLCurly{}, DoubleRCurly{}),
            sep=[AnyNumberOfCommas{}, AnyNumberOfCommas{},],
               
        ];

        match r {
            Ok(result_collection) => assert!(expected_collection == result_collection),
            Err(_) => fail!("Expected successful collection comparison")
        }
    }
}
