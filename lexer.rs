use regex::Regex;


const MATCHERS: &'static [(&str,Token)] = &[
    // Punctuation
    ("^,",Token::Comma ),
    (r#"^;"#,Token::SemiColon ),
    // Operations
    (r#"^="#,Token::Assign),
    (r#"^\+"#,Token::Plus ),
    (r#"^\-"#,Token::Minus ),
    (r#"^\*"#,Token::Mult ),
    ("^/",Token::Div ),
    //Key words
    ("^int",Token::KInt ),
    ("^return",Token::KReturn ),
    // Values
    // (r#"^"[0-9a-zA-Z,.;]*""#,Token::LiteralString(String::new())),
    ("^[0-9][0-9]*",Token::LiteralNumber(0)),
    //Identifier
    ("^[a-zA-Z][0-9a-zA-Z]*",Token::Identifier(String::new())),
    // Delimeters
    (r#"^\}"#,Token::RCurly ),
    (r#"^\{"#,Token::LCurly ),
    (r#"^\]"#,Token::RBracket ),
    (r#"^\["#,Token::LBracket ),
    (r#"^\)"#,Token::RParen ),
    (r#"^\("#,Token::LParen ),
];




pub fn lex(input:String )-> Result<Vec<Token>,&'static str>{
    let mut tokens = Vec::new();
    let mut i = 0;
    let a  = 0;
    while i < input.len(){
        if input.chars().nth(i).unwrap() == ' '{
            i+=1;
            continue;
        }
        let prev = i; 
        for (r,t) in MATCHERS {
            let (result,offset) = match Regex::new( r ).unwrap().captures(&input[i..]) {

                Some(matched) => {

                    t.from_regex_result(
                        matched.get(0)
                        .unwrap()
                        .as_str()
                        .to_string()
                    )
                } ,
                _ => continue,
            };
            tokens.push(result);
            i+=offset;
        }
        if prev != i{
            continue;
        }
        return Err("No matches");
    }
    Ok( tokens ) 
}


