use crate::token::Token;
use crate::parse::Parsable;
use crate::collection_builder::{CollectionBuilder, parse_collection};
use std::cmp::PartialEq;

#[cfg(test)]
#[path = "../tests/test1.rs"]
mod iter_tests;

pub struct TokenIter {
    pub current: usize,
    pub tokens: Vec<Token>,
    size: usize,
    stack: Vec<usize>
}

pub trait IntoTokenIter {
    fn into_token_iter(self) -> TokenIter;
}

impl IntoTokenIter for Vec<Token> {
    fn into_token_iter(self) -> TokenIter {
        TokenIter::new(self)
    }
}

impl TokenIter {
    pub fn new(tokens: Vec<Token>) -> TokenIter {
        TokenIter {
            current: 0,
            size: tokens.len(),
            tokens,
            stack: vec![]
        }
    }

    pub fn parse<T>(&mut self) -> Result<T,String>
     where T : Parsable {
         T::parse(self)
    }

    pub fn expect(&mut self, token:Token) -> Result<Token, String>{
         let error = "iter has run out of bounds";
         let result = self.get_next().ok_or(error)?;
         let expected_error = format!(" expected {:?} but got {:?}", result, token);
         if result.is_same_variant(& token) {
             Ok(result)
         } else {
             Err(expected_error)
         }
    }

    pub fn attempt<T>(&mut self)-> Result<T, String>
    where T:Parsable{
        self.push();
        let result = self.parse::<T>();
        match result {
            Ok(_) => { 
                self.clean_pop();
                result
            },
            Err(_) => {
                self.pop();
                result
            }
        }

    }
     
    pub fn increment(&mut self) -> usize {
        self.current += 1;
        self.current
    }

    pub fn collection<T>(&mut self) -> CollectionBuilder<T> where T: Parsable + PartialEq + std::fmt::Debug {
        parse_collection::<T>(self)
    }

    pub fn push(&mut self) {
        self.stack.push(self.current);
    }
    
    pub fn clean_pop(&mut self){
        self.stack.pop();
    }

    pub fn pop(&mut self) -> Option<usize>{
        match self.stack.pop() {
            Some(c) => { 
                self.current = c;
                Some(c)
            } ,
            None => None
        }
    }

    pub fn peek<T>(&mut self) -> Result<T, String> where T: Parsable {
        self.push();
        let result = T::parse(self);
        self.pop();
        result
    }

    pub fn peek_token(&mut self, token:Token) -> Result<Token, String> {
         self.push(); 
         let result = self.expect(token);
         self.pop(); 
         result
    }

    pub fn get_next(&mut self) -> Option<Token>{
        let result = self.get();
        self.increment();
        result
    }

    fn get(&self) -> Option<Token> {
        if self.current < self.size {
            return Some(self.tokens[self.current].clone());
        }
        None
    }

}




