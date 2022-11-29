use crate::parse::Parsable;

impl <T> Parsable for Vec<T> 
where T: Parsable{
    fn parse(iter: &mut crate::iter::TokenIter) -> Result<Self, String> where Self: Sized {
        let mut results = vec![]; 
        while let Ok(r) = iter.parse::<T>() {
            results.push(r);
        };
        return Ok(results)
    }
}