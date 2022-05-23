use std::marker::PhantomData;
use crate::parse::Parsable;
use crate::iter::Iter;


#[cfg(test)]
#[path = "collection_builder_test.rs"]
mod collection_builder_tests;

pub struct CollectionBuilder<'a, T, L=Always, R=Always, S=Always> where T: Parsable,L: Parsable ,R : Parsable, S : Parsable{
    iter: &'a mut Iter,
    min_len: usize,
    max_len: usize,
    _phantom: (PhantomData<T>, PhantomData<L>, PhantomData<R>, PhantomData<S>),
}

pub fn parse_collection<T>(iter: &mut Iter) -> CollectionBuilder<T>
where T: Parsable{
    CollectionBuilder::<T>::new(iter)
}

impl <'a, T, L, R, S> CollectionBuilder<'a, T, L, R, S> where T: Parsable,L: Parsable ,R : Parsable, S:Parsable{

    pub fn new(iter: &'a mut Iter) -> CollectionBuilder<T,L,R,S> {
        Self {
            min_len: 0,
            max_len: usize::MAX, // is this problematic?
            _phantom:( PhantomData,PhantomData,PhantomData,PhantomData),
            iter
        }
    }

    pub fn from_other <T1, L1, R1, S1> (&mut self) -> CollectionBuilder<T1,L1, R1, S1> 
        where T1: Parsable,L1: Parsable,R1: Parsable,S1: Parsable {
            CollectionBuilder::new(self.iter)
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
            match self.iter.attempt::<T>() {
                Ok(col_elem) => match self.iter.attempt::<S>(){
                    Ok(_) => {
                        println!("col_elem ok, s ok");
                        results.push(col_elem);
                        continue
                    },
                    Err(_) => {
                        results.push(col_elem);
                        println!("col_elem ok, s NOT ok; break loop");
                        break;
                    }                
                },
                Err(_) => { 
                    println!("col_elem NOT ok, s not parsed; break loop");
                    break;
                },
            }
        }
        let r_rightdel = self.iter.parse::<R>();
        r_leftdel.and(r_rightdel).and(self.check_len_restrictions(results))
    }

    fn check_len_restrictions(&self, results: Vec<T>) -> Result<Vec<T>,String>{
            if results.len() < self.min_len {
                Err(format!("parsed node collection {} did not respect min len ({})",results.len(), self.min_len))
            }else if results.len() > self.max_len {
                Err(format!("parsed node collection {} did not respect max len ({})",results.len(), self.max_len))
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

