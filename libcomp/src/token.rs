use crate::iter::Iter;
#[derive(PartialEq,Default, Debug,Clone)]
pub struct LiteralStringValue  {
    value: String
}

impl From<String> for LiteralStringValue {
    fn from(s: String) -> Self {
        LiteralStringValue{value: s }
    }
}


#[derive(PartialEq,Default, Debug,Clone)]
pub struct LiteralIntValue  {
    value: String
}

impl From<String> for LiteralIntValue {
    fn from(s: String) -> Self {
        LiteralIntValue{value: s }
    }
}


#[derive(PartialEq,Default, Debug,Clone)]
pub struct IdentifierValue  {
    value: String
}

impl From<String> for IdentifierValue {
    fn from(s: String) -> Self {
        IdentifierValue{value: s }
    }
}


#[derive(Debug,Clone, PartialEq)]
pub enum Token  {
    // Operations
    Assign(String),
    Plus(String),
    Minus(String),
    Mult(String),
    Div(String),
    // KeyWords and Literals
    KInt(String),
    KReturn(String),
    // Literals
    LiteralString(String),
    LiteralInt(u32),
    //Identifier
    Identifier(String),
    // Delimeters
    RCurly(String),
    LCurly(String),
    RBracket(String),
    LBracket(String),
    RParen(String),
    LParen(String),
    Comma(String),
    //Punctuation
    SemiColon(String),
    // meta
    INVALID,
    EMPTY
}

impl Token {
    pub fn from_regex_result(&self, input: String)-> (Token,usize) {
        let token = match *self  {
            Token::LiteralString(_) => Token::LiteralString(input[1..input.len() - 1].to_string().into()),// remove  quotes around string
            Token::Identifier(_) => Token::Identifier(input.clone().into()),// remove  quotes around string
            Token::LiteralInt(_) => { 
                let s:u32 = input.parse().unwrap();
                Token::LiteralInt(s) 
            },
            _ => (*self).clone()
        };
        (token,input.len())
    }

    pub fn is_same_variant(&self, token: Token) -> bool {
        match *self {
            Token::LiteralString(_) => match token {
                Token::LiteralString(_) => true,
                _ => false,
            },
            Token::LiteralInt(_) => match token {
                Token::LiteralInt(_) => true,
                _ => false
            },
            //Identifier
            Token::Identifier(_) => match token {
                Token::Identifier(_) => true,
                _ => false
            },
            // Operations
            Token::Assign(_) |
            Token::Plus(_) |
            Token::Minus(_) |
            Token::Mult(_) |
            Token::Div(_) |
            // KeyWords and Literals
            Token::KInt(_) |
            Token::KReturn(_) |
            // Literals
            // Delimeters
            Token::RCurly(_) |
            Token::LCurly(_) |
            Token::RBracket(_) |
            Token::LBracket(_) |
            Token::RParen(_) |
            Token::LParen(_) |
            Token::Comma(_) |
            //Punctuation
            Token::SemiColon(_)| 
            // meta
            Token::INVALID|
            Token::EMPTY => * self == token,
        }
    }

    pub fn parse(&self, iter:&mut Iter) -> Result<Token, String> {
        let t = iter.next().ok_or(format!("No tokens left to parse"))?;
        if self.is_same_variant(t) {
            Ok(t)
        } else {
            Err(format!("Expected token {:?}, but got token {:?}", *self, t))
        }
    }
}

#[macro_export]
macro_rules! t {
    (,) => (Token::Comma(",".to_string())); 
    (;) => (Token::Comma(";".to_string())); 
    (=) => (Token::Assign("=".to_string())); 
    (+) => (Token::Plus("+".to_string())); 
    (-) => (Token::Minus("-".to_string())); 
    (*) => (Token::Mult("*".to_string())); 
    (/) => (Token::Div("/".to_string())); 
    (int) => (Token::KInt("int".to_string())); 
    (return) => (Token::KReturn("return".to_string())); 
    (litstr) => (Token::LiteralString(Default::default()));
    (litint) => (Token::LiteralInt(Default::default())); 
    (ident) => (Token::Identifier(Default::default())); 
    (r_paren) => (Token::RParen(")".to_string())); 
    (l_paren) => (Token::LParen("(".to_string())); 
    (r_curly) => (Token::RCurly("}".to_string())); 
    (l_curly) => (Token::LCurly("{".to_string())); 
    (r_bracket) => (Token::RBracket("]".to_string())); 
    (l_bracket) => (Token::LBracket("[".to_string())); 
    (empty) => (Token::EMPTY); 
    (invalid) => (Token::INVALID); 
}
