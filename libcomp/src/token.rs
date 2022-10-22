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
    Assign,
    Plus,
    Minus,
    Mult,
    Div,
    // KeyWords and Literals
    KInt,
    KReturn,
    // Literals
    LiteralString(String),
    LiteralInt(u32),
    //Identifier
    Identifier(String),
    // Delimeters
    RCurly,
    LCurly,
    RBracket,
    LBracket,
    RParen,
    LParen,
    Comma,
    //Punctuation
    SemiColon,
    // meta
    INVALID,
    EMPTY
}

impl Token {
    pub fn from_regex_result(&self, input: String)-> (Token,usize) {
        let token = match *self  {
            Token::LiteralString(_) => Token::LiteralString(input[1..input.len() - 1].to_string()),// remove  quotes around string
            Token::Identifier(_) => Token::Identifier(input.clone()),// remove  quotes around string
            Token::LiteralInt(_) => { 
                let s:u32 = input.parse().unwrap();
                Token::LiteralInt(s) 
            },
            _ => (*self).clone()
        };
        (token,input.len())
    }

    pub fn is_same_variant(&self, token: &Token) -> bool {
        match *self {
            Token::LiteralString(_) => matches!(token,Token::LiteralString(_)),
            Token::LiteralInt(_) =>matches!(token,Token::LiteralInt(_)),
            Token::Identifier(_) =>matches!(token,Token::Identifier(_)),

            // Operations
            Token::Assign |
            Token::Plus |
            Token::Minus |
            Token::Mult |
            Token::Div |
            // KeyWords and Literals
            Token::KInt |
            Token::KReturn |
            // Literals
            // Delimeters
            Token::RCurly |
            Token::LCurly |
            Token::RBracket |
            Token::LBracket |
            Token::RParen |
            Token::LParen |
            Token::Comma |
            //Punctuation
            Token::SemiColon| 
            // meta
            Token::INVALID|
            Token::EMPTY => * self == * token,
        }
    }
}

#[macro_export]
macro_rules! t {
    (,) => (Token::Comma); 
    (, def) => (","); 
    (;) => (Token::SemiColon); 
    (; def) => (";"); 
    (=) => (Token::Assign); 
    (= def) => ("="); 
    (+) => (Token::Plus); 
    (+ def) => ("+"); 
    (-) => (Token::Minus); 
    (- def) => ("-"); 
    (*) => (Token::Mult); 
    (* def) => ("*"); 
    (/) => (Token::Div); 
    (/ def) => ("/"); 
    (int) => (Token::KInt); 
    (int def) => ("int"); 
    (return) => (Token::KReturn); 
    (return def) => ("return"); 

    (litstr) => (Token::LiteralString(Default::default()));
    (litint) => (Token::LiteralInt(Default::default())); 
    (ident) => (Token::Identifier(Default::default())); 

    (litstr $value:expr) => (Token::LiteralString($value.to_string()));
    (litint $value:expr) => (Token::LiteralInt($value)); 
    (ident $value:expr) => (Token::Identifier($value.to_string())); 

    (r_paren) => (Token::RParen); 
    (r_paren def) => (")"); 
    (l_paren) => (Token::LParen); 
    (l_paren def) => ("("); 
    (r_curly) => (Token::RCurly); 
    (r_curly def) => ("}"); 
    (l_curly) => (Token::LCurly); 
    (l_curly def) => ("{"); 
    (r_bracket) => (Token::RBracket); 
    (r_bracket def) => ("]"); 
    (l_bracket) => (Token::LBracket); 
    (l_bracket def) => ("["); 
    (empty) => (Token::EMPTY); 
    (invalid) => (Token::INVALID); 

}

pub use t;
