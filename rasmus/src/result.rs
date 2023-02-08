use crate::instances::value::Val;

pub type CompResult = Result<Val, Trap>;

pub struct Trap;
