use std::marker::PhantomData;
use crate::parse::Parsable;
use crate::token::Token;
use crate::iter::Iter;
use crate::t; 
pub enum ParsableItem<T: Parsable> {
    Parsable(T),
    Token(Token),
}

impl <T: Parsable>  ParsableItem <T> {
    fn parse_from(&self, iter: &mut Iter) -> Result <Self, String> where Self: Sized {
         match * self  {
             ParsableItem::Parsable(_) => T::parse(iter).and_then(|p| Ok(ParsableItem::Parsable(p))),
             ParsableItem::Token(t) => t.parse(iter).and_then(|t| Ok(ParsableItem::Token(t))),
         }
    }

}

#[allow(dead_code)]
pub struct CollectionBuilder<'a, T> where T : Parsable{
    iter: &'a mut Iter,
    collection_item: ParsableItem<T>,
    delimiter: Option<(Token, Token)>,
    separator: Option<Token>,
    min_len: Option<usize>,
    phantom: PhantomData<T>,
}

#[allow(dead_code)]
impl  <'a, T> CollectionBuilder<'a,T>where T :Parsable{
    pub fn new(iter: &'a mut Iter, collection_item: ParsableItem<T>) -> CollectionBuilder<T> {
        Self {
            collection_item,
            delimiter: None,
            separator: None,
            min_len: None,
            phantom: PhantomData,
            iter
        }
    }

    fn delimiter(&mut self, left: Token, right: Token) -> &mut Self {
        self.delimiter = Some(( left, right ));
        self
    }

    fn separator(&mut self, sep: Token) -> &mut Self {
        self.separator = Some(sep);
        self
    }

    fn min_len(&mut self, min_len:usize ) -> &mut Self {
        self.min_len = Some(min_len);
        self
    }

    fn parse(&mut self) -> Result<Vec<ParsableItem<T>>,String> {
        let _ldel = match &self.delimiter {
            Some((left_delimiter,_)) => self.iter.expect(left_delimiter.clone())?,
            None => t!( empty ),
        };
        let mut results: Vec<ParsableItem<T>> = vec![];
        loop {
            let collection_element: Result<ParsableItem<T>,String> = match &self.collection_item {
                ParsableItem::Parsable(_) => self.iter.peek::<T>().and_then(|c| Ok(ParsableItem::Parsable(c))),
                ParsableItem::Token(t) => self.iter.peek_token(t.clone()).and_then(|t| Ok(ParsableItem::Token(t))),
            };
            match collection_element {
                Ok(ce) => {
                    match &self.separator {
                        Some(sep) =>  {
                            let sep:Result<Token,String> = self.iter.peek_token(sep.clone());
                            match sep {
                                Ok(_) => {
                                    // one for the peeked result, another for the peeked seprator
                                    self.iter.increment();
                                    self.iter.increment();
                                    continue;
                                },
                                Err(_)=> { break; }
                            }
                        },
                        None => results.push(ce)
                    }
                },
                Err(_) => break,
            }
        }
        let _rdel = match &self.delimiter {
            Some((_, right_delimiter)) => Some ( self.iter.expect(right_delimiter.clone())? ),
            None => None
        };
        Ok(results)
    }
}

