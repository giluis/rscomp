use crate::iter::Iter;

pub trait Parsable {
    fn parse(iter: &mut Iter) -> Result <Self, String> where Self: Sized;
}
