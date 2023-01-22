use crate::{parse::Parsable, token::Token};

pub enum Error
{
    LexerError(LexerError),
    ParserError(ParserError),
}

pub enum LexerError {
    UnrecognizedToken(char),
}

pub enum ParserError 
{
    CouldNotParseType(CouldNotParseTypeError),
}


pub struct CouldNotParseTypeError {
    msg: String
}

impl  CouldNotParseTypeError where {

    fn new_conjunct(tried_to_parse: &str, failure_reason: &str) -> Self {
        CouldNotParseTypeError {  msg: format!("Could not parse {}, because failed to parse branch {}", tried_to_parse, failure_reason) }
    }

    fn new_disjunct(tried_to_parse: &str, v: &[&str]) -> Self {
        let mut failure_reason = "".to_string();
        for s in v {
            failure_reason.push(','); 
            failure_reason.push_str(s);
        }

        return CouldNotParseTypeError {  msg: format!("Could not parse {}, because failed to parse any branch ({})", tried_to_parse, failure_reason) }
    }

}

pub enum ResponsibleForParsingFailure<R> where R: Parsable {
    Token(Token), 
    Type(R)
}

impl <R> std::fmt::Display for ResponsibleForParsingFailure<R> where R: Parsable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let (type_of_responsible,identifier_of_responsible )= match self.responsible {
        //     Self::Token(t) => ("leaf", t) ,
        //     Self::Type(_) => ("type", R::name())
        // }; 
        write!(f, "")
    }
}


impl  std::fmt::Display for CouldNotParseTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}



// impl <T,R> std::fmt::Display for ParserError<T,R> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, ": {}", info)
//     }
// }


// impl std::fmt::Display for ParserError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             ParserError::NoMoreTokens(info) => ( write!(f, "NoMoreTokensError: {}", info)),
//             ParserError::CouldNotParseType(info) => (),
//         };
//     }
// }
