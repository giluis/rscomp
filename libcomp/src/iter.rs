use crate::token::Token;
use crate::parse::Parsable;
use crate::collection_builder::{CollectionBuilder, ParsableItem};

#[cfg(test)]
#[path = "iter_tests.rs"]
mod iter_tests;

pub struct Iter {
    current: usize,
    tokens: Vec<Token>,
    size: usize,
    stack: Vec<usize>
}


#[allow(dead_code)]
impl Iter {

    fn new(tokens: Vec<Token>) -> Iter {
        Iter {
            current: 0,
            size: tokens.len(),
            tokens,
            stack: vec![]
        }
    }

    fn parse<T>(&mut self, item: ParsableItem<T>) -> Result<ParsableItem<T>,String>
     where T : Parsable {
         match item  {
             ParsableItem::Parsable(_) => T::parse(self).and_then(|p| Ok(ParsableItem::Parsable(p))),
             ParsableItem::Token(t) => t.parse(self).and_then(|t|Ok(ParsableItem::Token(t))),
         }
    }

    fn expect(&mut self, token:Token) -> Result<Token, String>{
         let error = format!( "token {:?} could not be found here ", token);
         let result = self.next().ok_or(error)?;
         let expected_error = format!(" expected {:?} but got {:?}", result, token);
         if result.is_same_variant(token) {
             Ok(result)
         } else {
             Err(expected_error)
         }
    }

    fn increment(&mut self) -> usize {
        self.current += 1;
        self.current
    }

    fn collection<'a,T>(&'a mut self, ci: ParsableItem<T>) -> CollectionBuilder<'a,T> where T: Parsable {
        let a: CollectionBuilder<'a,T> =  CollectionBuilder::new(self, ci);
        return a; 
    }

    fn push(&mut self) {
        self.stack.push(self.current);
    }

    fn pop(&mut self) -> Option<usize>{
        match self.stack.pop() {
            Some(c) => { 
                self.current = c;
                Some(c)
            } ,
            None => None
        }
    }

    fn peek<T>(&mut self, item: ParsableItem<T>) -> Result<T, String> where T: Parsable {
        self.push();
        let result = T::parse(self);
        self.pop();
        return result;
    }

    fn peek_token(&mut self, token:Token) -> Result<Token, String> {
         self.push(); 
         let result = self.expect(token);
         self.pop(); 
         result
    }

    pub fn next(&mut self) -> Option<Token>{
        let result = self.get();
        self.increment();
        return result;
    }

    fn get(&self) -> Option<Token> {
        if self.current < self.size {
            return Some(self.tokens[self.current].clone());
        }
        return None;
    }

}


