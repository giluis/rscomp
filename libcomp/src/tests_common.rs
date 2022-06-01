use crate::token::Token;
use crate::iter::Iter;
use crate::t; 
use crate::parse::Parsable;

pub struct TestStruct {
    pub int_type: String,
    pub var_name: String,
}

impl TestStruct {

    pub fn from_string(string: &str) -> Self {
        TestStruct {
            int_type: t!( int def ).to_string(),
            var_name: string.to_string()
        }
    }
}

impl Parsable for TestStruct {
    fn parse(iter: &mut Iter) -> Result<TestStruct, String>{

        let var_type = match iter.expect(t!( int ))? {
            Token::KInt(int) => int,
            _ =>panic!("Internal error, should be \"int\""), 
        };

        let ident_str = match iter.expect(t!( ident ))? {
            Token::Identifier(ident_str)=> ident_str,
            _ => panic!("Internal error, should be ident_str"),
        };

        Ok(TestStruct { int_type: var_type,  var_name: ident_str})
    }
}

pub struct LBracket {}
impl Parsable for LBracket {
    fn parse(iter: &mut Iter) -> Result<Self, String> {
        iter.expect(t!( l_bracket )).map(|_|Self{/* no fields*/})
    }
}

pub struct RBracket {}
impl Parsable for RBracket {
    fn parse(iter: &mut Iter) -> Result<Self, String> {
        iter.expect(t!( r_bracket )).map(|_|Self{/* no fields*/})
    }
}


pub struct DoubleLBracket {}
impl Parsable for DoubleLBracket {
    fn parse(iter: &mut Iter) -> Result<Self, String> {
        iter.expect(t!( l_bracket ))
            .and(iter.expect(t!( l_bracket )))
            .map(|_|Self{/* no fields*/})
    }
}

pub struct DoubleRBracket {}
impl Parsable for DoubleRBracket {
    fn parse(iter: &mut Iter) -> Result<Self, String> {
        iter.expect(t!( r_bracket ))
            .and(iter.expect(t!( r_bracket )))
            .map(|_|Self{/* no fields*/})
    }
}

pub struct LParen {}
impl Parsable for LParen {
    fn parse(iter: &mut Iter) -> Result<Self, String> {
        iter.expect(t!( l_paren ))
            .map(|_|Self{/* no fields*/})
    }
}

pub struct RParen {}
impl Parsable for RParen {
    fn parse(iter: &mut Iter) -> Result<Self, String> {
        iter.expect(t!( r_paren ))
            .map(|_|Self{/* no fields*/})
    }
}

pub struct Comma {}
impl Parsable for Comma {
    fn parse(iter: &mut Iter) -> Result<Self, String> {
        iter.expect(t!( , ))
            .map(|_|Self{/* no fields*/})
    }
}


pub struct DoubleComma {}
impl Parsable for DoubleComma {
    fn parse(iter: &mut Iter) -> Result<Self, String> {
        iter.expect(t!( , ))
            .and(iter.expect(t!( , )))
            .map(|_|Self{/* no fields*/})
    }
}


pub struct AnyNumberOfCommas {}
impl Parsable for AnyNumberOfCommas {
    fn parse(iter: &mut Iter) -> Result<Self, String> {
        iter.collection::<Comma>()
                        .min_len(1)
                        .parse()
                        .and(Ok(AnyNumberOfCommas{}))
    }
}

pub struct LCurly {}
impl Parsable for LCurly {
    fn parse(iter: &mut Iter) -> Result<Self, String> {
        iter.expect(t!( l_curly ))
            .map(|_|Self{/* no fields*/})
    }
}

pub struct RCurly {}
impl Parsable for RCurly {
    fn parse(iter: &mut Iter) -> Result<Self, String> {
        iter.expect(t!( r_curly ))
            .map(|_|Self{/* no fields*/})
    }
}

pub struct DoubleLCurly {}
impl Parsable for DoubleLCurly {
    fn parse(iter: &mut Iter) -> Result<Self, String> {
        iter.expect(t!( l_curly ))
            .and(iter.expect(t!( l_curly )))
            .map(|_|Self{/* no fields*/})
    }
}

pub struct DoubleRCurly {}
impl Parsable for DoubleRCurly {
    fn parse(iter: &mut Iter) -> Result<Self, String> {
        iter.expect(t!( r_curly ))
            .and(iter.expect(t!( r_curly )))
            .map(|_|Self{/* no fields*/})
    }
}

