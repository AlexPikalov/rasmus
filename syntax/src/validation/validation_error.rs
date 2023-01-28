use crate::types::LocalIdx;

pub enum ValidationError {
    NoLocalFound(LocalIdx),
    InsufficientOperandStackForInstruction,
    CannotFindRefFuncInValidationContext,
}

pub type ValidationResult<T> = Result<T, ValidationError>;
