use crate::token::Token;

pub struct Iter {
    current: usize,
    tokens: Vec<Token>,
    size: usize
}


impl Iter {

    fn new(tokens: Vec<Token>) -> Iter {
        let size = tokens.len();
        Iter {
            current: 0,
            tokens,
            size,
        }
    }

    fn parse<T>(&mut self) -> Result<u32,String>
     where T : Parsable {
        return T::parse(self);
    }

    fn expect(&mut self, token:Token) -> Result<Token, String>{
         let error = format!( "token {:?} could not be found here ", token);
         self.get().ok_or(error)
    }

    fn next(&mut self) -> Option<Token>{
        let result = self.get();
        self.current +=1;
        return result;
    }

    fn get(&self) -> Option<Token> {
        if self.current < self.size {
            return Some(self.tokens[self.current].clone());
        }
        return None;
    }

}

trait Parsable {
    fn parse<T>(iter: &mut Iter) -> Result <T, String>;
}
