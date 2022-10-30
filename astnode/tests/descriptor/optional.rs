use astnode::AstNode;
use libcomp::iter::{IntoTokenIter, TokenIter};
use libcomp::parse::Parsable;
use libcomp::token::{t, Token};

#[derive(AstNode)]
pub struct Punct {

    #[stateless_leaf(Token::KInt)]
    kint: Option<Token>,

    ident1: Option<Identifier>,

    #[stateful_leaf(Token::Identifier)]
    ident3: Option<String>,
}


/// impl Parsable for Type  {
///    
/// fn parse(iter:&mut Iter) -> Result<TestEnum, String> {
///           
///    let ident = match iter.attempt::<Identifier>(){
///         Ok(DoubleComma) => Some((DoubleComma)),
///         Err(_) => None, 
///
///    };
///    let comma = match iter.peek_token(Token::LitInt(Default::default())) {
///         Ok(Token::LitInt(LitInt)) => {
///             lreturn Ok(TestEnum::LitInt(LitInt))
///         },
///         Err(_) => (),
///    };
///    match iter.peek_token(Token::SemiColon) {
///         Ok(Token::SemiColon) => {
///             lreturn Ok(TestEnum::SemiColon(Token::SemiColon))
///         },
///         Err(_) => (),
///    };
///    return Err("could not parse any of the variants for this sum node".to_string())
/// }
/// 




fn main() {
    let mut iter = vec![
        t!(=),
        ].into_token_iter();
    let result = iter.parse::<Punct>();
    match result {
        Ok(Punct::EqualSign(Token::Assign)) => (),
        Ok(_) => panic!("Expect DoubleComma variant, but didn't get that "), // internal error: token should always be t! (int) when result is OK
        _ => panic!("Expecte Ok Result"),
    }

    let mut iter = vec![
        t!(,),
        ].into_token_iter();
    let result = iter.parse::<Punct>();
    match result {
        Ok(Punct::SemiOrComma(SemiOrComma::Comma(Token::Comma))) => (),
        Ok(_) => panic!("Expect DoubleComma variant, but didn't get that "), // internal error: token should always be t! (int) when result is OK
        _ => panic!("Expecte Ok Result"),
    }
    // assert!(currentBefore + 1 == iter.current );

    let mut iter = vec![
        t!(;),
        ].into_token_iter();
    let result = iter.parse::<Punct>();
    match result {
        Ok(Punct::SemiOrComma(SemiOrComma::Semi(Token::SemiColon))) => (),
        Ok(_) => panic!("Expect Punct Punct::SemiOrComma(SemiOrComma::Semi(Token::SemiColon)) variant, but didn't get that "), // internal error: token should always be t! (int) when result is OK
        _ => panic!("Expected Ok Result"),
    }
}
