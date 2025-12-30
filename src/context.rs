use crate::{Args, AtError, AtResult};

pub trait AtContext {

    fn exec(&self) -> AtResult<'static> {
        Err(AtError::NotSupported)
    }

    fn query(&mut self) -> AtResult<'static> {
        Err(AtError::NotSupported)
    }
    
    fn test(&mut self) -> AtResult<'static> {
        Err(AtError::NotSupported)
    }

    fn set(&mut self, _args: Args) -> AtResult<'static> {
        Err(AtError::NotSupported)
    }

}