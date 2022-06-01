use regex::Regex;
use crate::token::{ Token,t };

pub fn lex(input:String )-> Result<Vec<Token>,&'static str>{
    let matchers= vec![
        // Punctuation
        ("^,", t!( , )),
        (r#"^;"#, t!( ; )),
        // Operations
        (r#"^="#, t!( = )),
        (r#"^\+"#, t!( + )),
        (r#"^\-"#, t!( - )),
        (r#"^\*"#, t!( * )),
        ("^/", t!( / )),
        //Key words
        ("^int", t!( int )),
        ("^return", t!( return )),
        // Values
        // (r#"^"[0-9a-zA-Z,.;]*""#,Token::LiteralString(String::new()))
        ("^[0-9][0-9]*", t!( litint )),
        //Identifier
        ("^[a-zA-Z][0-9a-zA-Z]*", t!( litstr )),
        // Delimeters
        (r#"^\}"#, t!( r_curly )),
        (r#"^\{"#, t!( l_curly )),
        (r#"^\]"#, t!( r_bracket )),
        (r#"^\["#, t!( l_bracket )),
        (r#"^\)"#, t!( r_paren )),
        (r#"^\("#, t!( l_paren )),
    ];

    let mut tokens = Vec::new();
    let mut i = 0;
    while i < input.len(){
        if input.chars().nth(i).unwrap() == ' '{
            i+=1;
            continue;
        }
        let prev = i; 
        for (r,t) in &matchers {
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


