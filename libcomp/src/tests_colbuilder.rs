#[cfg(test)]
mod collection_builder_tests {
    use super::super::*;
    use crate::t; 
    use crate::tests_common::*;
    use crate::token::Token;

    #[test]
    fn parse_nodel_nosep(){
        let expected_varname1 = "var1";
        let expected_varname2 = "var2";
        let expected_varname3 = "var3";
        let mut iter = Iter::new(vec![
                t!( int ),
                t!( ident expected_varname1),
                t!( int ),
                t!( ident expected_varname2),
                t!( int ),
                t!( ident expected_varname3),

        ]);
        let r = iter.collection::<TestStruct>().parse();
        let expected_vec = vec![
            TestStruct::from_string(expected_varname1),
            TestStruct::from_string(expected_varname2),
            TestStruct::from_string(expected_varname3),
        ];

        assert!(r.is_ok());
        let result_vec = r.unwrap();
        // assert these are equal; this prevents accidental pass due to zip behaviour
        assert!(expected_vec.len() == result_vec.len());
        for (e, r) in expected_vec.iter().zip(result_vec.iter()){
            assert_eq!(e.int_type, r.int_type);
            assert_eq!(e.int_type, r.int_type);
        }

    }


    #[test]
    fn parse_del_nosep(){
        let expected_varname1 = "var1";
        let expected_varname2 = "var2";
        let mut iter = Iter::new(vec![
                t!( l_paren ),
                t!( int ),
                t!( ident expected_varname1 ),
                t!( int ),
                t!( ident expected_varname2 ),
                t!( r_paren ),

        ]);

        let r = iter.collection::<TestStruct>()
                                    .delimiter::<LParen,RParen>()
                                    .parse();
        let expected_vec = vec![
            TestStruct::from_string(expected_varname1),
            TestStruct::from_string(expected_varname2),
        ];
    
        assert!(r.is_ok());
        let result_vec = r.unwrap();
        // assert these are equal; this prevents accidental pass due to zip behaviour
        assert!(expected_vec.len() == result_vec.len());
        for (e, r) in expected_vec.iter().zip(result_vec.iter()){
            assert_eq!(e.int_type, r.int_type);
            assert_eq!(e.var_name, r.var_name);
        }
    }

    #[test]
    fn parse_nodel_sep(){
        let expected_varname1 = "var1";
        let expected_varname2 = "var2";
        let expected_varname3 = "var2";
        let mut iter = Iter::new(vec![
                t!( int ),
                t!( ident expected_varname1 ),
                t!( , ),
                t!( int ),
                t!( ident expected_varname2 ),
                t!( , ),
                t!( int ),
                t!( ident expected_varname3 ),
        ]);

        let r = iter.collection::<TestStruct>()
                                .separator::<Comma>()
                                .parse();
        let expected_vec = vec![
            TestStruct::from_string(expected_varname1),
            TestStruct::from_string(expected_varname2),
            TestStruct::from_string(expected_varname3),
        ];

        assert!(r.is_ok());
        let result_vec = r.unwrap();
        // assert these are equal; this prevents accidental pass due to zip behaviour
        assert!(expected_vec.len() == result_vec.len());
        for (e, r) in expected_vec.iter().zip(result_vec.iter()){
            assert_eq!(e.int_type, r.int_type);
            assert_eq!(e.var_name, r.var_name);
        }
    }

    #[test]
    fn parse_nodel_sep_fail(){
        let expected_varname1 = "var1";
        let expected_varname2 = "var2";
        let expected_varname3 = "var2";
        let mut iter = Iter::new(vec![
                t!( int ),
                t!( ident expected_varname1 ),
                t!( , ),
                t!( int ),
                t!( ident expected_varname2 ),
                t!( , ),
                t!( int ),
                t!( ident expected_varname3 ),
        ]);

        let r = iter.collection::<TestStruct>()
                                // missing separator
                                // .separator::<Comma>()
                                .parse();
        let expected_vec = vec![
            TestStruct::from_string(expected_varname1),
            TestStruct::from_string(expected_varname2),
            TestStruct::from_string(expected_varname3),
        ];

        // result is not error since it parses the first int var1
        // assert!(r.is_err()); 
        let result_vec = r.unwrap();
        // assert these are not equal, since this test is supposed to fail;
        assert!(expected_vec.len() != result_vec.len());
        for (e, r) in expected_vec.iter().zip(result_vec.iter()){
            assert_eq!(e.int_type, r.int_type);
            assert_eq!(e.var_name, r.var_name);
        }
    }


    #[test]
    fn parse_del_sep(){
        let expected_varname1 = "var1";
        let expected_varname2 = "var2";
        let expected_varname3 = "var2";
        let mut iter = Iter::new(vec![
                t!( l_curly ),
                t!( int ),
                t!( ident expected_varname1 ),
                t!( , ),
                t!( int ),
                t!( ident expected_varname2 ),
                t!( , ),
                t!( int ),
                t!( ident expected_varname3 ),
                t!( r_curly ),
        ]);

        let r = iter.collection::<TestStruct>()
                                .delimiter::<LCurly,RCurly>()
                                .separator::<Comma>()
                                .parse();

        let expected_vec = vec![
            TestStruct::from_string(expected_varname1),
            TestStruct::from_string(expected_varname2),
            TestStruct::from_string(expected_varname3),
        ];

        assert!(r.is_ok());
        let result_vec = r.unwrap();
        // assert these are equal; this prevents accidental pass due to zip behaviour
        assert!(expected_vec.len() == result_vec.len());
        for (e, r) in expected_vec.iter().zip(result_vec.iter()){
            assert_eq!(e.int_type, r.int_type);
            assert_eq!(e.var_name, r.var_name);
        }
    }

    #[test]
    fn parse_del_sep_fail(){
        let expected_varname1 = "var1";
        let expected_varname2 = "var2";
        let expected_varname3 = "var2";
        let mut iter = Iter::new(vec![
                t!( l_curly ),
                t!( int ),
                t!( ident expected_varname1 ),
                t!( , ),
                t!( int ),
                t!( ident expected_varname2 ),
                t!( , ),
                t!( int ),
                t!( ident expected_varname3 ),
                t!( r_paren ),
        ]);

        let r = iter.collection::<TestStruct>()
                                .delimiter::<LCurly,RCurly>()
                                .separator::<Comma>()
                                .parse();

        assert!(r.is_err());
    }

    #[test]
    fn parse_successful_min_len(){
        let expected_varname1 = "var1";
        let expected_varname2 = "var2";
        let expected_varname3 = "var2";
        let mut iter = Iter::new(vec![
                t!( l_bracket ),
                t!( int ),
                t!( ident expected_varname1 ),
                t!( , ),
                t!( int ),
                t!( ident expected_varname2 ),
                t!( , ),
                t!( int ),
                t!( ident expected_varname3 ),
                t!( r_bracket ),
        ]);

        let r = iter.collection::<TestStruct>()
                                .delimiter::<LBracket,RBracket>()
                                .separator::<Comma>()
                                .min_len(2)
                                .parse();

        let expected_vec = vec![
            TestStruct::from_string(expected_varname1),
            TestStruct::from_string(expected_varname2),
            TestStruct::from_string(expected_varname3),
        ];

        assert!(r.is_ok());
        let result_vec = r.unwrap();
        // assert these are equal; this prevents accidental pass due to zip behaviour
        assert!(expected_vec.len() == result_vec.len());
        for (e, r) in expected_vec.iter().zip(result_vec.iter()){
            assert_eq!(e.int_type, r.int_type);
            assert_eq!(e.var_name, r.var_name);
        }
    }

    #[test]
    fn parse_failed_min_len(){
        let expected_varname1 = "var1";
        let mut iter = Iter::new(vec![
                t!( l_bracket ),
                t!( int ),
                t!( ident expected_varname1 ),
                t!( r_bracket ),
        ]);

        let min_len = 2;
        let r = iter.collection::<TestStruct>()
                                .delimiter::<LBracket,RBracket>()
                                .min_len(min_len)
                                .parse();
        match r {
            Ok(_)=> assert!(false), //TODO fail test
            Err(msg) => assert_eq!(msg,format!("parsed node collection {} did not respect min len ({})",1,min_len) ),
        };
    }


    #[test]
    fn parse_failed_min_len4(){
        let expected_varname1 = "var1";
        let mut iter = Iter::new(vec![
                t!( l_bracket ),
                t!( int ),
                t!( ident expected_varname1 ),
                t!( , ),
                t!( int ),
                t!( ident expected_varname1 ),
                t!( , ),
                t!( int ),
                t!( ident expected_varname1 ),
                t!( r_bracket ),
        ]);

        let min_len = 4;
        let r = iter.collection::<TestStruct>()
                                .delimiter::<LBracket,RBracket>()
                                .separator::<Comma>()
                                .min_len(min_len)
                                .parse();
        match r {
            Ok(_)=> assert!(false), //TODO fail test
            Err(msg) => assert_eq!(msg,format!("parsed node collection {} did not respect min len ({})",3,min_len) ),
        };
    }

    // inspired by double object destructuring
    #[test]
    fn parse_doubledel_singlesep(){
        let expected_varname1 = "var1";
        let expected_varname2 = "var2";
        let expected_varname3 = "var2";
        let mut iter = Iter::new(vec![
                t!( l_bracket ),
                t!( l_bracket ),
                t!( int ),
                t!( ident expected_varname1 ),
                t!( , ),
                t!( int ),
                t!( ident expected_varname2 ),
                t!( , ),
                t!( int ),
                t!( ident expected_varname3 ),
                t!( r_bracket ),
                t!( r_bracket ),
        ]);

        let r = iter.collection::<TestStruct>()
                                .delimiter::<DoubleLBracket,DoubleRBracket>()
                                .separator::<Comma>()
                                .min_len(2)
                                .parse();

        let expected_vec = vec![
            TestStruct::from_string(expected_varname1),
            TestStruct::from_string(expected_varname2),
            TestStruct::from_string(expected_varname3),
        ];

        assert!(r.is_ok());
        let result_vec = r.unwrap();
        // assert these are equal; this prevents accidental pass due to zip behaviour
        assert!(expected_vec.len() == result_vec.len());
        for (e, r) in expected_vec.iter().zip(result_vec.iter()){
            assert_eq!(e.int_type, r.int_type);
            assert_eq!(e.var_name, r.var_name);
        }
    }

    // inspired by lua string composition
    #[test]
    fn parse_nodel_doublesep(){
        let expected_varname1 = "var1";
        let expected_varname2 = "var2";
        let expected_varname3 = "var2";
        let mut iter = Iter::new(vec![
                t!( int ),
                t!( ident expected_varname1 ),
                t!( , ),
                t!( , ),
                t!( int ),
                t!( ident expected_varname2 ),
                t!( , ),
                t!( , ),
                t!( int ),
                t!( ident expected_varname3 ),
        ]);

        let r = iter.collection::<TestStruct>()
                                .separator::<DoubleComma>()
                                .min_len(2)
                                .parse();

        let expected_vec = vec![
            TestStruct::from_string(expected_varname1),
            TestStruct::from_string(expected_varname2),
            TestStruct::from_string(expected_varname3),
        ];

        assert!(r.is_ok());
        let result_vec = r.unwrap();
        // assert these are equal; this prevents accidental pass due to zip behaviour
        assert!(expected_vec.len() == result_vec.len());
        for (e, r) in expected_vec.iter().zip(result_vec.iter()){
            assert_eq!(e.int_type, r.int_type);
            assert_eq!(e.var_name, r.var_name);
        }
    }

    // inspired by javascript arrays

    #[test]
    fn parse_nodel_complex_sep(){
        let expected_varname1 = "var1";
        let expected_varname2 = "var2";
        let expected_varname3 = "var2";
        let mut iter = Iter::new(vec![
                t!( int ),
                t!( ident expected_varname1 ),
                t!( , ),
                t!( , ),
                t!( , ),
                t!( int ),
                t!( ident expected_varname2 ),
                t!( , ),
                t!( int ),
                t!( ident expected_varname3 ),
        ]);
        let r = iter.collection::<TestStruct>()
                                .separator::<AnyNumberOfCommas>()
                                .min_len(2) //casually testing min len with complex sep
                                .parse();

        let expected_vec = vec![
            TestStruct::from_string(expected_varname1),
            TestStruct::from_string(expected_varname2),
            TestStruct::from_string(expected_varname3),
        ];

        let result_vec = r.unwrap();
        // assert these are equal; this prevents accidental pass due to zip behaviour
        assert!(expected_vec.len() == result_vec.len());
        for (e, r) in expected_vec.iter().zip(result_vec.iter()){
            assert_eq!(e.int_type, r.int_type);
            assert_eq!(e.var_name, r.var_name);
        }
    }

    #[test]
    fn parse_doubledel_complex_sep(){
        let expected_varname1 = "var1";
        let expected_varname2 = "var2";
        let expected_varname3 = "var2";
        let mut iter = Iter::new(vec![
                t!( l_curly ),
                t!( l_curly ),
                t!( int ),
                t!( ident expected_varname1 ),
                t!( , ),
                t!( , ),
                t!( , ),
                t!( int ),
                t!( ident expected_varname2 ),
                t!( , ),
                t!( int ),
                t!( ident expected_varname3 ),
                t!( r_curly ),
                t!( r_curly ),
        ]);
        let r = iter.collection::<TestStruct>()
                                .delimiter::<DoubleLCurly,DoubleRCurly>()
                                .separator::<AnyNumberOfCommas>()
                                .min_len(2) //casually testing min len with complex sep
                                .parse();

        let expected_vec = vec![
            TestStruct::from_string(expected_varname1),
            TestStruct::from_string(expected_varname2),
            TestStruct::from_string(expected_varname3),
        ];

        let result_vec = r.unwrap();
        // assert these are equal; this prevents accidental pass due to zip behaviour
        assert!(expected_vec.len() == result_vec.len());
        for (e, r) in expected_vec.iter().zip(result_vec.iter()){
            assert_eq!(e.int_type, r.int_type);
            assert_eq!(e.var_name, r.var_name);
        }
    }
}
