#[derive(Debug,Clone)]
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
    LiteralNumber(u32),
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
        fn from_regex_result(&self, input: String)-> (Token,usize) {
            let token = match *self  {
                Token::LiteralString(_) => Token::LiteralString(input[1..input.len() - 1].to_string()),// remove  quotes around string
                Token::Identifier(_) => Token::Identifier(input.clone()),// remove  quotes around string
                Token::LiteralNumber(_) => Token::LiteralNumber(input.parse().unwrap()),
                _ => (*self).clone()
            };
            (token,input.len())
        }
    }

