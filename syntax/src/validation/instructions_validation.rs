use crate::check;
use crate::instructions::{InstructionType, InstructionType as I};
use crate::types::*;

use super::context::ValidationContext;
use super::validation_error::{ValidationError, ValidationResult};

// NOTE: it is used for validating a sequence of instructions
// example: `(i64.const 0) (i32.const 1)` i32.add is invalid
// in a sequence despite each of individual instruction may be valid.

const MAX_ALLOWED_LINE_IDX: u8 = 32;

#[derive(Debug, PartialEq)]
pub enum OpdType {
    Strict(ValType),
    Any,
    AnyOf(Vec<ValType>),
}

impl OpdType {
    pub fn matches(&self, other: ValType) -> bool {
        match self {
            Self::Strict(self_strict) => *self_strict == other,
            Self::Any => true,
            Self::AnyOf(options) => options.iter().any(|opt| *opt == other),
        }
    }

    fn is_strict(&self) -> bool {
        match self {
            &Self::Strict(_) => true,
            _ => false,
        }
    }
}

pub struct OperandStack {
    stack: Vec<ValType>,
}

impl OperandStack {
    pub fn new() -> Self {
        OperandStack { stack: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        OperandStack {
            stack: Vec::with_capacity(capacity),
        }
    }

    // returns true if valid and applied and false if invalid
    pub fn validate_and_apply(&mut self, mut instruction: StackType) -> bool {
        let is_valid = instruction.inputs.iter().all(|operand_type| {
            self.stack
                .pop()
                .map(|stack_val_type| operand_type.matches(stack_val_type))
                .unwrap_or(false)
        });

        if is_valid {
            self.stack.append(&mut instruction.outputs);
        }

        is_valid
    }
}

/// Stack types describe how instructions manipulate the operand stack.
#[derive(Debug, PartialEq)]
pub struct StackType {
    /// Operand types which an instruction pops from a Stack.
    pub inputs: Vec<OpdType>,
    /// Operand types which an instruction pushes back to a Stack as a result.
    pub outputs: Vec<ValType>,
}

// TODO: refactor followig method to return ValidationResult type instead of bool
pub fn is_instr_sequence_valid(
    sequence: Vec<InstructionType>,
    ctx: ValidationContext,
) -> ValidationResult<()> {
    let mut operand_stack = OperandStack::with_capacity(sequence.len());

    for ref instr in sequence {
        let instr_stack_type = get_stack_type_for_instruction(instr, &ctx)?;
        let is_valid = operand_stack.validate_and_apply(instr_stack_type);

        if !is_valid {
            return Err(ValidationError::InsufficientOperandStackForInstruction);
        }
    }

    Ok(())
}

pub fn is_instruction_valid(instruction: &InstructionType, ctx: &ValidationContext) -> bool {
    unimplemented!()
}

pub fn get_stack_type_for_instruction(
    instruction: &InstructionType,
    ctx: &ValidationContext,
) -> ValidationResult<StackType> {
    let stack_type = match instruction {
        // Numeric Instructions
        // t.const c
        I::I32Const(_) => StackType {
            inputs: vec![],
            outputs: vec![ValType::NumType(NumType::I32)],
        },
        I::I64Const(_) => StackType {
            inputs: vec![],
            outputs: vec![ValType::NumType(NumType::I64)],
        },
        I::F32Const(_) => StackType {
            inputs: vec![],
            outputs: vec![ValType::NumType(NumType::F32)],
        },
        I::F64Const(_) => StackType {
            inputs: vec![],
            outputs: vec![ValType::NumType(NumType::F64)],
        },

        // t.unop
        I::I32Clz | I::I32Ctz | I::I32Popcnt => StackType {
            inputs: vec![OpdType::Strict(ValType::NumType(NumType::I32))],
            outputs: vec![ValType::NumType(NumType::I32)],
        },
        I::I64Clz | I::I64Ctz | I::I64Popcnt => StackType {
            inputs: vec![OpdType::Strict(ValType::NumType(NumType::I64))],
            outputs: vec![ValType::NumType(NumType::I64)],
        },
        I::F32Abs
        | I::F32Neg
        | I::F32Ceil
        | I::F32Floor
        | I::F32Trunc
        | I::F32Nearest
        | I::F32Sqrt => StackType {
            inputs: vec![OpdType::Strict(ValType::NumType(NumType::F32))],
            outputs: vec![ValType::NumType(NumType::F32)],
        },
        I::F64Abs
        | I::F64Neg
        | I::F64Ceil
        | I::F64Floor
        | I::F64Trunc
        | I::F64Nearest
        | I::F64Sqrt => StackType {
            inputs: vec![OpdType::Strict(ValType::NumType(NumType::F64))],
            outputs: vec![ValType::NumType(NumType::F64)],
        },
        // t.binop

        // t.testop
        I::I32Eqz => StackType {
            inputs: vec![OpdType::Strict(ValType::NumType(NumType::I32))],
            outputs: vec![ValType::NumType(NumType::I32)],
        },
        I::I64Eqz => StackType {
            inputs: vec![OpdType::Strict(ValType::NumType(NumType::I64))],
            outputs: vec![ValType::NumType(NumType::I32)],
        },
        I::I32Add
        | I::I32Sub
        | I::I32Mul
        | I::I32DivS
        | I::I32DivU
        | I::I32RemS
        | I::I32RemU
        | I::I32And
        | I::I32Or
        | I::I32Xor
        | I::I32Shl
        | I::I32ShrS
        | I::I32ShrU
        | I::I32Rotl
        | I::I32Rotr => StackType {
            inputs: vec![
                OpdType::Strict(ValType::NumType(NumType::I32)),
                OpdType::Strict(ValType::NumType(NumType::I32)),
            ],
            outputs: vec![ValType::NumType(NumType::I32)],
        },
        I::I64Add
        | I::I64Sub
        | I::I64Mul
        | I::I64DivS
        | I::I64DivU
        | I::I64RemS
        | I::I64RemU
        | I::I64And
        | I::I64Or
        | I::I64Xor
        | I::I64Shl
        | I::I64ShrS
        | I::I64ShrU
        | I::I64Rotl
        | I::I64Rotr => StackType {
            inputs: vec![
                OpdType::Strict(ValType::NumType(NumType::I64)),
                OpdType::Strict(ValType::NumType(NumType::I64)),
            ],
            outputs: vec![ValType::NumType(NumType::I64)],
        },
        I::F32Add | I::F32Sub | I::F32Mul | I::F32Div | I::F32Min | I::F32Max | I::F32Copysign => {
            StackType {
                inputs: vec![
                    OpdType::Strict(ValType::NumType(NumType::F32)),
                    OpdType::Strict(ValType::NumType(NumType::F32)),
                ],
                outputs: vec![ValType::NumType(NumType::F32)],
            }
        }
        I::F64Add | I::F64Sub | I::F64Mul | I::F64Div | I::F64Min | I::F64Max | I::F64Copysign => {
            StackType {
                inputs: vec![
                    OpdType::Strict(ValType::NumType(NumType::F64)),
                    OpdType::Strict(ValType::NumType(NumType::F64)),
                ],
                outputs: vec![ValType::NumType(NumType::F64)],
            }
        }
        // t.relop
        I::I32Eq
        | I::I32Ne
        | I::I32LtS
        | I::I32LtU
        | I::I32GtS
        | I::I32GtU
        | I::I32LeS
        | I::I32LeU
        | I::I32GeS
        | I::I32GeU => StackType {
            inputs: vec![
                OpdType::Strict(ValType::NumType(NumType::I32)),
                OpdType::Strict(ValType::NumType(NumType::I32)),
            ],
            outputs: vec![ValType::NumType(NumType::I32)],
        },
        I::I64Eq
        | I::I64Ne
        | I::I64LtS
        | I::I64LtU
        | I::I64GtS
        | I::I64GtU
        | I::I64LeS
        | I::I64LeU
        | I::I64GeS
        | I::I64GeU => StackType {
            inputs: vec![
                OpdType::Strict(ValType::NumType(NumType::I64)),
                OpdType::Strict(ValType::NumType(NumType::I64)),
            ],
            outputs: vec![ValType::NumType(NumType::I32)],
        },
        I::F64Eq | I::F64Ne | I::F64Lt | I::F64Gt | I::F64Le | I::F64Ge => StackType {
            inputs: vec![
                OpdType::Strict(ValType::NumType(NumType::F64)),
                OpdType::Strict(ValType::NumType(NumType::F64)),
            ],
            outputs: vec![ValType::NumType(NumType::I32)],
        },
        // t.cvtop
        I::I32WrapI64 => StackType {
            inputs: vec![OpdType::Strict(ValType::NumType(NumType::I32))],
            outputs: vec![ValType::NumType(NumType::I64)],
        },
        I::I32Extend8S | I::I32Extend16S => StackType {
            inputs: vec![OpdType::Strict(ValType::NumType(NumType::I32))],
            outputs: vec![ValType::NumType(NumType::I32)],
        },
        I::I64Extend8S | I::I64Extend16S | I::I64Extend32S => StackType {
            inputs: vec![OpdType::Strict(ValType::NumType(NumType::I64))],
            outputs: vec![ValType::NumType(NumType::I64)],
        },
        I::I32TruncF32S | I::I32TruncF32U | I::I32TruncSatF32S | I::I32TruncSatF32U => StackType {
            inputs: vec![OpdType::Strict(ValType::NumType(NumType::F32))],
            outputs: vec![ValType::NumType(NumType::I32)],
        },
        I::I32TruncF64S | I::I32TruncF64U | I::I32TruncSatF64S | I::I32TruncSatF64U => StackType {
            inputs: vec![OpdType::Strict(ValType::NumType(NumType::F64))],
            outputs: vec![ValType::NumType(NumType::I32)],
        },
        I::I64TruncF32S | I::I64TruncF32U | I::I64TruncSatF32S | I::I64TruncSatF32U => StackType {
            inputs: vec![OpdType::Strict(ValType::NumType(NumType::F32))],
            outputs: vec![ValType::NumType(NumType::I64)],
        },
        I::I64TruncF64S | I::I64TruncF64U | I::I64TruncSatF64S | I::I64TruncSatF64U => StackType {
            inputs: vec![OpdType::Strict(ValType::NumType(NumType::F64))],
            outputs: vec![ValType::NumType(NumType::I64)],
        },
        I::F32ConvertI32S | I::F32ConvertI32U => StackType {
            inputs: vec![OpdType::Strict(ValType::NumType(NumType::I32))],
            outputs: vec![ValType::NumType(NumType::F32)],
        },
        I::F32ConvertI64S | I::F32ConvertI64U => StackType {
            inputs: vec![OpdType::Strict(ValType::NumType(NumType::I64))],
            outputs: vec![ValType::NumType(NumType::F32)],
        },
        I::F64ConvertI32S | I::F64ConvertI32U => StackType {
            inputs: vec![OpdType::Strict(ValType::NumType(NumType::I32))],
            outputs: vec![ValType::NumType(NumType::F64)],
        },
        I::F64ConvertI64S | I::F64ConvertI64U => StackType {
            inputs: vec![OpdType::Strict(ValType::NumType(NumType::I64))],
            outputs: vec![ValType::NumType(NumType::F64)],
        },
        I::F32DemoteF64 => StackType {
            inputs: vec![OpdType::Strict(ValType::NumType(NumType::F64))],
            outputs: vec![ValType::NumType(NumType::F32)],
        },
        I::F64PromoteF32 => StackType {
            inputs: vec![OpdType::Strict(ValType::NumType(NumType::F32))],
            outputs: vec![ValType::NumType(NumType::F64)],
        },
        I::I32ReinterpretF32 => StackType {
            inputs: vec![OpdType::Strict(ValType::NumType(NumType::F32))],
            outputs: vec![ValType::NumType(NumType::I32)],
        },
        I::I64ReinterpretF64 => StackType {
            inputs: vec![OpdType::Strict(ValType::NumType(NumType::F64))],
            outputs: vec![ValType::NumType(NumType::I64)],
        },
        I::F32ReinterpretI32 => StackType {
            inputs: vec![OpdType::Strict(ValType::NumType(NumType::I32))],
            outputs: vec![ValType::NumType(NumType::F32)],
        },
        I::F64ReinterpretI64 => StackType {
            inputs: vec![OpdType::Strict(ValType::NumType(NumType::I64))],
            outputs: vec![ValType::NumType(NumType::F64)],
        },
        // Ref Instructions
        I::RefNull(ref ref_type) => StackType {
            inputs: vec![],
            outputs: vec![ValType::RefType(ref_type.clone())],
        },
        I::RefIsNull => StackType {
            inputs: vec![OpdType::AnyOf(ValType::get_ref_types())],
            outputs: vec![ValType::NumType(NumType::I32)],
        },
        I::RefFunc(ref func_idx) => {
            if ctx.funcs.get(func_idx.0 .0 as usize).is_none()
                || ctx.refs.get(func_idx.0 .0 as usize).is_none()
            {
                return Err(ValidationError::CannotFindRefFuncInValidationContext);
            }

            StackType {
                inputs: vec![],
                outputs: vec![ValType::RefType(RefType::FuncRef)],
            }
        }

        // Vector Instructions
        // v128.const
        I::V128Const(_) => StackType {
            inputs: vec![],
            outputs: vec![ValType::v128()],
        },
        // v128.vvunop
        I::V128Not => StackType {
            inputs: vec![OpdType::Strict(ValType::v128())],
            outputs: vec![ValType::v128()],
        },
        // v128.vvbinop
        I::V128And | I::V128AndNot | I::V128Or | I::V128Xor => StackType {
            inputs: vec![
                OpdType::Strict(ValType::v128()),
                OpdType::Strict(ValType::v128()),
            ],
            outputs: vec![ValType::v128()],
        },
        // v128.vvternop
        I::V128Bitselect => StackType {
            inputs: vec![
                OpdType::Strict(ValType::v128()),
                OpdType::Strict(ValType::v128()),
                OpdType::Strict(ValType::v128()),
            ],
            outputs: vec![ValType::v128()],
        },
        // v128.vvtestop
        I::V128AnyTrue => StackType {
            inputs: vec![
                OpdType::Strict(ValType::v128()),
                OpdType::Strict(ValType::v128()),
                OpdType::Strict(ValType::v128()),
            ],
            outputs: vec![ValType::v128()],
        },
        // i8x16.swizzle
        I::I8x16Swizzle => StackType {
            inputs: vec![
                OpdType::Strict(ValType::v128()),
                OpdType::Strict(ValType::v128()),
            ],
            outputs: vec![ValType::v128()],
        },
        I::I8x16Shuffle(lane_indexes) => {
            for lane_idx in lane_indexes {
                if lane_idx.0 >= MAX_ALLOWED_LINE_IDX {
                    return Err(ValidationError::LaneIndexIsOutOfRange {
                        value: lane_idx.0,
                        max_allowed: 32,
                    });
                }
            }

            StackType {
                inputs: vec![
                    OpdType::Strict(ValType::v128()),
                    OpdType::Strict(ValType::v128()),
                ],
                outputs: vec![ValType::v128()],
            }
        }
        // shape.splat (more details about 'shape' - https://webassembly.github.io/spec/core/valid/instructions.html#vector-instructions)
        I::I8x16Splat | I::I16x8Splat | I::I32x4Splat => StackType {
            inputs: vec![OpdType::Strict(ValType::i32())],
            outputs: vec![ValType::v128()],
        },
        I::I64x2Splat => StackType {
            inputs: vec![OpdType::Strict(ValType::i64())],
            outputs: vec![ValType::v128()],
        },
        I::F32x4Splat => StackType {
            inputs: vec![OpdType::Strict(ValType::f32())],
            outputs: vec![ValType::v128()],
        },
        I::F64x2Splat => StackType {
            inputs: vec![OpdType::Strict(ValType::f64())],
            outputs: vec![ValType::v128()],
        },
        // shape.extract_lane_sx
        I::I8x16ExtractLaneS(lane_idx) | I::I8x16ExtractLaneU(lane_idx) => check! {
            extract_lane i8, 16, lane_idx
        },
        I::I16x8ExtractLaneS(lane_idx) | I::I16x8ExtractLaneU(lane_idx) => check! {
            extract_lane i16, 8, lane_idx
        },
        I::I32x4ExtractLane(lane_idx) => check! {
            extract_lane i32, 4, lane_idx
        },
        I::I64x2ExtractLane(lane_idx) => check! {
            extract_lane i64, 2, lane_idx
        },
        I::F32x4ExtractLane(lane_idx) => check! {
            extract_lane f32, 4, lane_idx
        },
        I::F64x2ExtractLane(lane_idx) => check! {
            extract_lane f64, 2, lane_idx
        },
        I::I8x16ReplaceLane(lane_idx) => check! {
            replace_lane i8, 16, lane_idx
        },
        I::I16x8ReplaceLane(lane_idx) => check! {
            replace_lane i16, 8, lane_idx
        },
        I::I32x4ReplaceLane(lane_idx) => check! {
            replace_lane i32, 4, lane_idx
        },
        I::I64x2ReplaceLane(lane_idx) => check! {
            replace_lane i64, 2, lane_idx
        },
        I::F32x4ReplaceLane(lane_idx) => check! {
            replace_lane f32, 4, lane_idx
        },
        I::F64x2ReplaceLane(lane_idx) => check! {
            replace_lane f64, 2, lane_idx
        },
        I::I8x16Abs
        | I::I8x16Neg
        | I::I16x8Abs
        | I::I16x8Neg
        | I::I32x4Abs
        | I::I32x4Neg
        | I::I64x2Abs
        | I::I64x2Neg
        | I::F32x4Abs
        | I::F32x4Neg
        | I::F64x2Abs
        | I::F64x2Neg
        | I::F32x4Sqrt
        | I::F64x2Sqrt
        | I::F32x4Ceil
        | I::F64x2Ceil
        | I::F32x4Floor
        | I::F64x2Floor
        | I::F32x4Trunc
        | I::F64x2Trunc
        | I::F32x4Nearest
        | I::F64x2Nearest
        | I::I8x16Popcnt => StackType {
            inputs: vec![OpdType::Strict(ValType::v128())],
            outputs: vec![ValType::v128()],
        },
        // I::I8x18Abs | I::I8x18Neg =>
        // shape.vunop
        // vunop = uiunop | vfunop | popcnt

        // _ => unimplemented!(),
        // _ => unimplemented!(),
    };

    Ok(stack_type)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_macro() {
        let allowed_lane_idx = 1u8;
        let not_allowed_lane = 20u8;

        {
            let test_check = || {
                let lane_idx = LaneIdx(allowed_lane_idx);
                Ok(check! {
                    extract_lane i8, 16, lane_idx
                })
            };

            assert_eq!(
                test_check().unwrap(),
                StackType {
                    inputs: vec![OpdType::Strict(ValType::v128())],
                    outputs: vec![ValType::i32()],
                }
            );
        }

        {
            let test_check = || {
                let lane_idx = LaneIdx(not_allowed_lane);
                Ok(check! {
                    extract_lane i8, 16, lane_idx
                })
            };

            assert_eq!(
                test_check().unwrap_err(),
                ValidationError::LaneIndexIsOutOfRange {
                    value: not_allowed_lane,
                    max_allowed: 15
                }
            );
        }
    }
}
