use std::ops::{FromResidual,Try};

pub struct DisjunctResultWrapper<T, E>(pub Result<T,E>); 

impl<T, E> FromResidual<DisjunctResultWrapper<T, E>> for DisjunctResultWrapper<T, E> {
    fn from_residual(residual: DisjunctResultWrapper<T, E>) -> Self {
        return residual;
    }
}

impl<T, E> FromResidual<DisjunctResultWrapper<T, E>> for Result<T, E> {
    fn from_residual(residual: DisjunctResultWrapper<T, E>) -> Self {
            match residual{
            DisjunctResultWrapper (  Ok(r) ) => Ok(r),
            _ => unreachable!("E cannot be instantiated")
        }
    }
}

impl<T, E> Try for DisjunctResultWrapper<T, E> {
    type Output = E;
    type Residual = DisjunctResultWrapper<T,E>;

    fn from_output(output: Self::Output) -> Self {
        DisjunctResultWrapper(  Err(output))
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self.0 {
            Ok(value)  => std::ops::ControlFlow::Break(DisjunctResultWrapper (  Ok(value) )),
            Err(err_msg) => std::ops::ControlFlow::Continue(err_msg)
        }
    }
}




impl<T, E> DisjunctResultWrapper<T, E> {
    pub fn map<P>(self, construction_function: fn(T)-> P) -> DisjunctResultWrapper<P,E> {
        match self.0 {
            Ok(result) => DisjunctResultWrapper(Ok(construction_function(result))), 
            Err(err) => DisjunctResultWrapper::<P,E>(Err(err))
        }
    }
}