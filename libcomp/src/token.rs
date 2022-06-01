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

#[macro_export]
macro_rules! t {
    (,) => (Token::Comma(",".to_string())); 
    (, def) => (","); 
    (;) => (Token::SemiColon(";".to_string())); 
    (; def) => (";"); 
    (=) => (Token::Assign("=".to_string())); 
    (= def) => ("="); 
    (+) => (Token::Plus("+".to_string())); 
    (+ def) => ("+"); 
    (-) => (Token::Minus("-".to_string())); 
    (- def) => ("-"); 
    (*) => (Token::Mult("*".to_string())); 
    (* def) => ("*"); 
    (/) => (Token::Div("/".to_string())); 
    (/ def) => ("/"); 
    (int) => (Token::KInt("int".to_string())); 
    (int def) => ("int"); 
    (return) => (Token::KReturn("return".to_string())); 
    (return def) => ("return"); 

    (litstr) => (Token::LiteralString(Default::default()));
    (litint) => (Token::LiteralInt(Default::default())); 
    (ident) => (Token::Identifier(Default::default())); 

    (litstr $value:expr) => (Token::LiteralString($value.to_string()));
    (litint $value:expr) => (Token::LiteralInt($value)); 
    (ident $value:expr) => (Token::Identifier($value.to_string())); 

    (r_paren) => (Token::RParen(")".to_string())); 
    (r_paren def) => (")"); 
    (l_paren) => (Token::LParen("(".to_string())); 
    (l_paren def) => ("("); 
    (r_curly) => (Token::RCurly("}".to_string())); 
    (r_curly def) => ("}"); 
    (l_curly) => (Token::LCurly("{".to_string())); 
    (l_curly def) => ("{"); 
    (r_bracket) => (Token::RBracket("]".to_string())); 
    (r_bracket def) => ("]"); 
    (l_bracket) => (Token::LBracket("[".to_string())); 
    (l_bracket def) => ("["); 
    (empty) => (Token::EMPTY); 
    (invalid) => (Token::INVALID); 

}

pub use t;
