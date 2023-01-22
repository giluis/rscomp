use crate::iter::TokenIter;

pub trait Parsable {
    fn parse(iter: &mut TokenIter) -> Result <Self, String> where Self: Sized;
    
    fn name() -> String {
        std::any::type_name::<Self>().to_string()
    }

}
