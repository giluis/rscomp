use std::marker::PhantomData;
use crate::parse::Parsable;
use crate::token::Token;
use crate::iter::Iter;
use crate::t; 


#[cfg(test)]
#[path = "collection_builder_test.rs"]
mod collection_builder_tests;

#[allow(dead_code)]
pub struct CollectionBuilder<'a, T, L=Always, R=Always, S=Always> where T: Parsable,L: Parsable ,R : Parsable, S : Parsable{
    iter: &'a mut Iter,
    min_len: usize,
    max_len: usize,
    _phantom: (PhantomData<T>, PhantomData<L>, PhantomData<R>, PhantomData<S>),
}

pub fn parse_collection<T>(iter: &mut Iter) -> CollectionBuilder<T>
where T: Parsable{
    CollectionBuilder::<T>::from_iter(iter)
}

#[allow(dead_code)]
impl <'a, T, L, R, S> CollectionBuilder<'a, T, L, R, S> where T: Parsable,L: Parsable ,R : Parsable, S:Parsable{

    pub fn from_iter(iter: &'a mut Iter) -> CollectionBuilder<T,L,R,S> {
        Self {
            min_len: 0,
            max_len: usize::MAX, // is this problematic?
            _phantom:( PhantomData,PhantomData,PhantomData,PhantomData),
            iter
        }
    }

    pub fn from_other <T1, L1, R1, S1> (&mut self) -> CollectionBuilder<T1,L1, R1, S1> 
        where T1: Parsable,L1: Parsable,R1: Parsable,S1: Parsable {
            return CollectionBuilder::from_iter(self.iter); 
    }

    pub fn delimiter<L1,R1>(&mut self) -> CollectionBuilder<T,L1,R1,S>
    where L1: Parsable, R1: Parsable{
        self.from_other::<T,L1,R1,S>()
    }

    pub fn separator<S1>(&mut self) ->CollectionBuilder<T,L,R,S1>
    where S1: Parsable{
        self.from_other::<T,L,R,S1>()
    }

    pub fn min_len(&mut self, min_len:usize ) -> &mut Self {
        self.min_len = min_len;
        self
    }

    pub fn max_len(&mut self, max_len:usize ) -> &mut Self {
        self.max_len = max_len;
        self
    }


    pub fn parse(&mut self) -> Result<Vec<T>,String> {
        let r_leftdel = self.iter.parse::<L>();
        let mut results: Vec<T> = vec![];
        loop {
            println!("\n == new loop iteration");
            self.iter.push();
            println!("  current index: {:?}",self.iter.current);
            println!("  current token: {:?}",self.iter.tokens[self.iter.current]);
            let collection_element = self.iter.attempt::<T>(); 
            if collection_element.is_ok() {
                println!("  elem ok index: {:?}",self.iter.current);
                println!("  elem ok cur token: {:?}",self.iter.tokens[self.iter.current]);
                let r_sep = self.iter.peek::<S>();
                println!("  sep result status: {:?}",r_sep.is_ok());
                results.push(collection_element.unwrap());
                if r_sep.is_ok(){
                    println!("  sep was ok");
                    let _ = self.iter.parse::<S>();
                    self.iter.clean_pop();
                    continue;
                } else {
                    println!("  sep was NOT ok");
                    // self.iter.pop();
                    break;
                }
            } else if collection_element.is_err() {

                println!("  elem was NOT ok");
                self.iter.pop();
                break;
            }
        }
        let r_rightdel = self.iter.parse::<R>();
        r_leftdel.and(r_rightdel).and(self.check_len_restrictions(results))
    }

    fn check_len_restrictions(&self, results: Vec<T>) -> Result<Vec<T>,String>{
            if results.len() < self.min_len {
                Err(format!("parsed node collection {} did not respect min len ({})",results.len(), self.min_len))
            }else if results.len() > self.max_len {
                Err(format!("tokens parsed {} did not respect max len ({})",results.len(), self.max_len))
            } else {
                Ok(results)
            }
    }
}

pub struct Always {}

impl Parsable for Always {
    fn parse(_iter: &mut Iter) -> Result<Always, String> {
        Ok(Self{/*fields*/})
    }
}

pub struct Never {}

impl Parsable for Never {
    fn parse(_iter: &mut Iter) -> Result<Never, String> {
        Err("iter should stop".to_string())
    }
}

