use crate::iter::TokenIter;

pub trait Parsable {
    fn parse(iter: &mut TokenIter) -> Result <Self, String> where Self: Sized;
}
