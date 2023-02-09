use crate::instances::value::Val;

pub type CompResult = Result<Val, Trap>;

pub type RResult<T> = Result<T, Trap>;

pub struct Trap;
