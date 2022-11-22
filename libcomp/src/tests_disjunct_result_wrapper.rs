

#[cfg(test)]
mod iter_tests {
    use super::super::*;
    use crate::disjunct_result_wrapper::DisjunctResultWrapper;
    use crate::token::Token;
    use crate::lexer::lex;
    use crate::t; 


    fn dummy_fn<T,E : Default>(r:Result<T,E>) -> Result <T, E>{
        let a =   DisjunctResultWrapper(r)?;
        Err(E::default())
    }

    #[test]
    fn test_it_breaks_when_inner_result_is_ok(){
        let input = Result::<u32,String>::Ok(3);
        let result = dummy_fn(input.clone());
        assert!(input == result);
    }

    fn test_it_continues_when_inner_result_is_err(){
        let input = Result::<u32,String>::Err("Some error msg".to_string());
        let result = dummy_fn(input.clone());
        assert!(result.is_err());
    }
}
