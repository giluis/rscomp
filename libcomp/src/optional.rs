use crate::parse::Parsable;
use crate::iter::TokenIter;


impl <T> Parsable for Option<T> 
where T: Parsable{
    fn parse(iter: &mut TokenIter) -> Result<Self, String> where Self: Sized {
        let r = iter.parse::<T>();
        match r {
            Ok(r) => Ok(Some(r)),
            Err(_) => unimplemented!("Error types are necessary here")
        }
    }
}
