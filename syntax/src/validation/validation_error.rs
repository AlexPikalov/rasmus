use crate::types::LocalIdx;

// TODO: try to add more debugging information to each option
#[derive(Debug, PartialEq)]
pub enum ValidationError {
    NoLocalFound(LocalIdx),
    InsufficientOperandStackForInstruction,
    CannotFindRefFuncInValidationContext,
    LaneIndexIsOutOfRange { value: u8, max_allowed: u8 },
}

pub type ValidationResult<T> = Result<T, ValidationError>;
