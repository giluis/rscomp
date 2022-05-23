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
            Token::EMPTY => * self == * token,
        }
    }
}

pub const COMMA_DEFAULT_STRING:&str = ","; 
pub const SEMICOLON_DEFAULT_STRING:&str = ";"; 
pub const ASSIGN_DEFAULT_STRING:&str = "="; 
pub const PLUS_DEFAULT_STRING:&str = "+"; 
pub const MINUS_DEFAULT_STRING:&str = "-"; 
pub const MULT_DEFAULT_STRING:&str = "*"; 
pub const DIV_DEFAULT_STRING:&str = "/"; 
pub const KINT_DEFAULT_STRING:&str = "int"; 
pub const KRETURN_DEFAULT_STRING:&str = "return"; 

pub const RPAREN_DEFAULT_STRING:&str = ")"; 
pub const LPAREN_DEFAULT_STRING:&str = "("; 
pub const RCURLY_DEFAULT_STRING:&str = "}"; 
pub const LCURLY_DEFAULT_STRING:&str = "{"; 
pub const RBRACKET_DEFAULT_STRING:&str = "]"; 
pub const LBRACKET_DEFAULT_STRING:&str = "["; 

#[macro_export]
macro_rules! t {
    (,) => (crate::token::Token::Comma(crate::token::COMMA_DEFAULT_STRING.to_string())); 
    (;) => (crate::token::Token::SemiColon(crate::token::SEMICOLON_DEFAULT_STRING.to_string())); 
    (=) => (crate::token::Token::Assign(crate::token::ASSIGN_DEFAULT_STRING.to_string())); 
    (+) => (crate::token::Token::Plus(crate::token::PLUS_DEFAULT_STRING.to_string())); 
    (-) => (crate::token::Token::Minus(crate::token::MINUS_DEFAULT_STRING.to_string())); 
    (*) => (crate::token::Token::Mult(crate::token::MULT_DEFAULT_STRING.to_string())); 
    (/) => (crate::token::Token::Div(crate::token::DIV_DEFAULT_STRING.to_string())); 
    (int) => (crate::token::Token::KInt(crate::token::KINT_DEFAULT_STRING.to_string())); 
    (return) => (crate::token::Token::KReturn(crate::token::KRETURN_DEFAULT_STRING.to_string())); 

    (litstr) => (crate::token::Token::LiteralString(Default::default()));
    (litint) => (crate::token::Token::LiteralInt(Default::default())); 
    (ident) => (crate::token::Token::Identifier(Default::default())); 

    (litstr $value:expr) => (crate::token::Token::LiteralString($value.to_string()));
    (litint $value:expr) => (crate::token::Token::LiteralInt($value)); 
    (ident $value:expr) => (crate::token::Token::Identifier($value.to_string())); 

    (r_paren) => (crate::token::Token::RParen(crate::token::RPAREN_DEFAULT_STRING.to_string())); 
    (l_paren) => (crate::token::Token::LParen(crate::token::LPAREN_DEFAULT_STRING.to_string())); 
    (r_curly) => (crate::token::Token::RCurly(crate::token::RCURLY_DEFAULT_STRING.to_string())); 
    (l_curly) => (crate::token::Token::LCurly(crate::token::LCURLY_DEFAULT_STRING.to_string())); 
    (r_bracket) => (crate::token::Token::RBracket(crate::token::RBRACKET_DEFAULT_STRING.to_string())); 
    (l_bracket) => (crate::token::Token::LBracket(crate::token::LBRACKET_DEFAULT_STRING.to_string())); 
    (empty) => (crate::token::Token::EMPTY); 
    (invalid) => (crate::token::Token::INVALID); 
}
