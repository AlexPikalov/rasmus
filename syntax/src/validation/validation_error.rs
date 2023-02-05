use crate::types::LocalIdx;

// TODO: try to add more debugging information to each option
#[derive(Debug, PartialEq)]
pub enum ValidationError {
    NoLocalFound(LocalIdx),
    InsufficientOperandStackForInstruction,
    CannotFindRefFuncInValidationContext,
    LaneIndexIsOutOfRange { value: u8, max_allowed: u8 },
    // Length of SelectVec argument sequence should be equal to 1
    InvalidSelectVecOperandSequence,
    LocalNotFound,
    GlobalNotFound,
    UnableToSetToConstGlobal,
    TableNotFound,
    // When do table.copy x y, table types of x and y must be the same
    UnableToCopyIncosistentTableTypes,
    ElemNotFound,
    // When init table elem type should be the same as a table's ref type
    WrongElemType,
    // When load a value memarg align should not be bigger than th bit width divided by 8
    MemargAlignTooBig,
    MemNotFound,
    LaneIdxTooBix,
    DataNotFound,
}

pub type ValidationResult<T> = Result<T, ValidationError>;
