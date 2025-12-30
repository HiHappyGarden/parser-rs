use crate::{Args, AtResult};

pub type ExecFn<T>  = fn(&mut T) -> AtResult<'static>;
pub type QueryFn<T> = fn(&mut T) -> AtResult<'static>;
pub type TestFn<T>  = fn(&mut T) -> AtResult<'static>;
pub type SetFn<T>   = fn(&mut T, Args) -> AtResult<'static>;

pub struct AtCommand<T> {
    pub name:  &'static str,
    pub exec:  Option<ExecFn<T>>,
    pub query: Option<QueryFn<T>>,
    pub test:  Option<TestFn<T>>,
    pub set:   Option<SetFn<T>>,
    pub help:  Option<&'static str>,
}

