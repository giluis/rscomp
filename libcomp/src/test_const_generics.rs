// use crate::token::Token;


// fn parse<const T:Token>() -> T {

// }


pub trait Parsable {
    fn parse(iter: &mut Vec<PossibleTokens>) -> Result <Self, String> where Self: Sized;
}


#[derive(PartialEq, Eq)]
pub enum PossibleTokens {
    Comma,
    Semi, 
    LiteralInt(u32),
}


pub struct Leaf<const Tokenn:PossibleTokens>;

impl <const T:PossibleTokens> Leaf<T>{
    const t:PossibleTokens = T;

    fn get()-> T {

    }
}



pub struct Punct {
    comma: Leaf<{PossibleTokens::Comma}>,
    lit_int: Leaf<{PossibleTokens::LiteralInt(4)}>
}

impl <const T:PossibleTokens> Parsable for Leaf<T>  {
    fn parse(iter:&mut Vec<PossibleTokens>) -> Result<Leaf<T>,String> {
        let a = iter.get(0);
        match a {
            Some(b) => if *b == T {return Ok(Leaf::<T>)} else {return Err("Did not get expected token".to_string())},
            None => return Err("no more tokens".to_string())
        }
    }
}


fn parse<T: Parsable>(iter: &mut Vec<PossibleTokens>) -> Result<T, String> {
    T::parse(iter)
}


fn main() -> (){
    let mut v = vec![PossibleTokens::Comma];
    let a = parse::<Leaf<{PossibleTokens::Comma}>>(&mut v);

}