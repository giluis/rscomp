#![allow(dead_code)]
#![allow(unused_variables)]

use std::marker::PhantomData;
use std::iter::Zip;
use std::slice::Iter;
use crate::parse::Parsable;
use crate::iter::TokenIter;
use std::cmp::PartialEq;


#[cfg(test)]
#[path = "tests_colbuilder.rs"]
mod collection_builder_tests;


pub struct Collection<T, L=(), R=(), S=()> 
where T: Parsable + PartialEq + std::fmt::Debug,
      L: Parsable + PartialEq + std::fmt::Debug ,
      R: Parsable + PartialEq + std::fmt::Debug,
      S: Parsable + PartialEq + std::fmt::Debug{
    elements: Vec<T>,
    right_del: R,
    left_del: L,
    sep: Vec<S>,
}

impl <T,L,R,S> PartialEq for Collection<T,L,R,S> 
where T: Parsable + PartialEq + std::fmt::Debug,
      L: Parsable + PartialEq + std::fmt::Debug,
      R: Parsable + PartialEq + std::fmt::Debug,
      S: Parsable + PartialEq + std::fmt::Debug{

    fn eq(&self, other: &Self) -> bool {
        // self.elements.iter().zip(other.elements.iter())
        self.len() == other.len() &&
        self.zip(other)
            .filter(|(s,o)| s == o)
            .next().is_some()
    }
}


#[macro_export]
macro_rules! collection {


    ([$($element:expr,)*], del=($left_del:expr, $right_del:expr), sep=[$($sep:expr,)*],) => {
        {
            let mut sep = vec![];
            $(
                sep.push($sep);
            )*
            let mut c = Collection::new(vec![], $left_del, $right_del, sep);
            $(
                c.push($element);
            )*
            c
        }
    };


    ([$($element:expr,)*], del=($left_del:expr, $right_del:expr)) => {
        {
            // let mut c = Collection {
            //     elements: vec![],
            //     left_del: $left_del,
            //     right_del: $right_del,
            //     sep: vec![],
            //     }
             
            let mut c = Collection::new(vec![],$left_del, $right_del, vec![]) ;
            $(
                c.push($element);
            )*
            c
        }
    };
    ([$($element:expr,)*], sep=[$($sep:expr,)*],) => {
        {
            let mut sep = vec![];
            $(
                sep.push($sep);
            )*
            let mut c = Collection::new(vec![], (), (), sep);
            $(
                c.push($element);
            )*
            c
        }
    };


    ($($element:expr,)* ) => {
        {
            let mut c = Collection {
                elements: vec![],
                left_del: (),
                right_del: (),
                sep: vec![()],
            };
            $(
                c.push($element);
            )*
            c
        }
    };

}

impl < T, L, R, S> Collection <T, L, R, S> 
where T: Parsable + PartialEq + std::fmt::Debug,
      L: Parsable + PartialEq + std::fmt::Debug,
      R: Parsable + PartialEq + std::fmt::Debug,
      S: Parsable + PartialEq + std::fmt::Debug
{

    fn len(&self) -> usize {
        self.elements.len()
    }

    fn zip<'a>(&'a self, other: &'a Self) -> Zip<Iter<T>,Iter<T>> {
        self.elements.iter().zip(other.elements.iter())
    }
    
    fn push(&mut self, element: T) -> &mut Self {
        self.elements.push(element);
        self
    }

    // fn map<P, F:FnMut(&T) -> P>(&self, function: F)-> Collection <P, L, R, S>
    // where P: Parsable + PartialEq + std::fmt::Debug{
    //     let new_elements = self.elements.iter().map(function).collect();
    //     Collection::new(new_elements, self.left_del.clone(), self.right_del, self.sep)
    // }
    //
    //
    //

    fn nosep(sep: (), leftdel: L, rightdel: R) -> Collection<T,L,R,()> 
    {
        Collection {
            elements: vec![], 
            left_del: leftdel,
            right_del: rightdel,
            sep: vec![()]
        }
    }

    fn empty() -> Collection<T,(),(),()> {
        Collection::new(vec![],(),(),vec![])
    }

    fn nodelsep(sep: Vec<S>) -> Collection<T,(),(),S>{
        Collection::new(vec![], (),(),vec![])
    }
    fn new(elements: Vec<T>, left_del: L, right_del: R, sep: Vec<S>) -> Self {
        Self {
            elements, 
            left_del,
            right_del,
            sep
        }
    }

}

pub struct CollectionBuilder<'a, T, L=(), R=(), S=()> 
where T: Parsable + PartialEq + std::fmt::Debug,
      L: Parsable + PartialEq + std::fmt::Debug,
      R: Parsable + PartialEq + std::fmt::Debug,
      S: Parsable + PartialEq + std::fmt::Debug
{
    iter: &'a mut TokenIter,
    min_len: usize,
    max_len: usize,
    _phantom: (PhantomData<T>,PhantomData<L>,PhantomData<R>,PhantomData<S>),
}

pub fn parse_collection<T>(iter: &mut TokenIter) -> CollectionBuilder<T>
where T: Parsable + PartialEq + std::fmt::Debug{
    CollectionBuilder::<T>::new(iter)
}

impl <'a, T, L, R, S> CollectionBuilder<'a, T, L, R, S> 
where T: Parsable + PartialEq + std::fmt::Debug,
      L: Parsable + PartialEq + std::fmt::Debug,
      R: Parsable + PartialEq + std::fmt::Debug,
      S: Parsable + PartialEq + std::fmt::Debug{

    pub fn new(iter: &'a mut TokenIter) -> CollectionBuilder<T,L,R,S> {
        Self {
            min_len: 0,
            max_len: usize::MAX, // is this problematic?
            iter,
            _phantom: (PhantomData, PhantomData, PhantomData, PhantomData),
        }
    }

    pub fn from_other <T1, L1, R1, S1> (&mut self) -> CollectionBuilder<T1,L1, R1, S1> 
        where T1: Parsable + PartialEq + std::fmt::Debug,
              L1: Parsable + PartialEq + std::fmt::Debug,
              R1: Parsable + PartialEq + std::fmt::Debug,
              S1: Parsable + PartialEq + std::fmt::Debug{
            CollectionBuilder::new(self.iter)
    }

    pub fn delimiter<L1,R1>(&mut self) -> CollectionBuilder<T,L1,R1,S>
    where L1: Parsable + PartialEq + std::fmt::Debug, 
          R1: Parsable + PartialEq + std::fmt::Debug{
        self.from_other::<T,L1,R1,S>()
    }

    pub fn separator<S1>(&mut self) ->CollectionBuilder<T,L,R,S1>
    where S1: Parsable + PartialEq + std::fmt::Debug{
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



    pub fn parse(&mut self) -> Result<Collection<T,L,R,S>,String> {
        let r_leftdel = self.iter.parse::<L>()?;
        let mut results: Vec<T> = vec![];
        let mut sep_results: Vec<S> = vec![];
        loop {
            match self.iter.attempt::<T>() {
                Ok(col_elem) => match self.iter.attempt::<S>(){
                    Ok(s) => {
                        sep_results.push(s); 
                        results.push(col_elem);
                        continue
                    },
                    Err(_) => {
                        results.push(col_elem);
                        break;
                    }                
                },
                Err(_) => { 
                    break;
                },
            }
        }
        let r_rightdel = self.iter.parse::<R>()?;
        // println!("r_leftdel: {:?}", r_leftdel);
        // println!("r_rightdel: {:?}", r_rightdel);
        // println!("sep_results: {:?}", sep_results);
        // println!("results: {:?}", results);
        self.check_len_restrictions(&results)?;
        Ok(Collection::new(results, r_leftdel, r_rightdel, sep_results))
    }

    fn check_len_restrictions(&self, results: &Vec<T>) -> Result<(),String>{
            if results.len() < self.min_len {
                Err(format!("parsed node collection {} did not respect min len ({})",results.len(), self.min_len))
            }else if results.len() > self.max_len {
                Err(format!("parsed node collection {} did not respect max len ({})",results.len(), self.max_len))
            } else {
                Ok(())
            }
    }
}

#[derive(PartialEq)]
pub struct Never {}


impl Parsable for Never {
    fn parse(_iter: &mut TokenIter) -> Result<Never, String> {
        Err("iter should stop".to_string())
    }
}


impl Parsable for () {
    fn parse(_iter: &mut TokenIter) -> Result<(), String> {
        Ok(())
    }
}
