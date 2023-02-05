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

pub fn get_stack_type_for_instruction(
    instruction: &InstructionType,
    ctx: &ValidationContext,
) -> ValidationResult<StackType> {
    let stack_type = match instruction {
        // Numeric Instructions
        // t.const c
        I::I32Const(_) => StackType {
            inputs: vec![],
            outputs: vec![ValType::i32()],
        },
        I::I64Const(_) => StackType {
            inputs: vec![],
            outputs: vec![ValType::i64()],
        },
        I::F32Const(_) => StackType {
            inputs: vec![],
            outputs: vec![ValType::f32()],
        },
        I::F64Const(_) => StackType {
            inputs: vec![],
            outputs: vec![ValType::f64()],
        },

        // t.unop
        I::I32Clz | I::I32Ctz | I::I32Popcnt => StackType {
            inputs: vec![OpdType::Strict(ValType::i32())],
            outputs: vec![ValType::i32()],
        },
        I::I64Clz | I::I64Ctz | I::I64Popcnt => StackType {
            inputs: vec![OpdType::Strict(ValType::i64())],
            outputs: vec![ValType::i64()],
        },
        I::F32Abs
        | I::F32Neg
        | I::F32Ceil
        | I::F32Floor
        | I::F32Trunc
        | I::F32Nearest
        | I::F32Sqrt => StackType {
            inputs: vec![OpdType::Strict(ValType::f32())],
            outputs: vec![ValType::f32()],
        },
        I::F64Abs
        | I::F64Neg
        | I::F64Ceil
        | I::F64Floor
        | I::F64Trunc
        | I::F64Nearest
        | I::F64Sqrt => StackType {
            inputs: vec![OpdType::Strict(ValType::f64())],
            outputs: vec![ValType::f64()],
        },
        // t.binop

        // t.testop
        I::I32Eqz => StackType {
            inputs: vec![OpdType::Strict(ValType::i32())],
            outputs: vec![ValType::i32()],
        },
        I::I64Eqz => StackType {
            inputs: vec![OpdType::Strict(ValType::i64())],
            outputs: vec![ValType::i32()],
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
                OpdType::Strict(ValType::i32()),
                OpdType::Strict(ValType::i32()),
            ],
            outputs: vec![ValType::i32()],
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
                OpdType::Strict(ValType::i64()),
                OpdType::Strict(ValType::i64()),
            ],
            outputs: vec![ValType::i64()],
        },
        I::F32Add | I::F32Sub | I::F32Mul | I::F32Div | I::F32Min | I::F32Max | I::F32Copysign => {
            StackType {
                inputs: vec![
                    OpdType::Strict(ValType::f32()),
                    OpdType::Strict(ValType::f32()),
                ],
                outputs: vec![ValType::f32()],
            }
        }
        I::F64Add | I::F64Sub | I::F64Mul | I::F64Div | I::F64Min | I::F64Max | I::F64Copysign => {
            StackType {
                inputs: vec![
                    OpdType::Strict(ValType::f64()),
                    OpdType::Strict(ValType::f64()),
                ],
                outputs: vec![ValType::f64()],
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
                OpdType::Strict(ValType::i32()),
                OpdType::Strict(ValType::i32()),
            ],
            outputs: vec![ValType::i32()],
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
                OpdType::Strict(ValType::i64()),
                OpdType::Strict(ValType::i64()),
            ],
            outputs: vec![ValType::i32()],
        },
        I::F32Eq | I::F32Ne | I::F32Lt | I::F32Gt | I::F32Le | I::F32Ge => StackType {
            inputs: vec![
                OpdType::Strict(ValType::f32()),
                OpdType::Strict(ValType::f32()),
            ],
            outputs: vec![ValType::i32()],
        },
        I::F64Eq | I::F64Ne | I::F64Lt | I::F64Gt | I::F64Le | I::F64Ge => StackType {
            inputs: vec![
                OpdType::Strict(ValType::f64()),
                OpdType::Strict(ValType::f64()),
            ],
            outputs: vec![ValType::i32()],
        },
        // t.cvtop
        I::I32WrapI64 => StackType {
            inputs: vec![OpdType::Strict(ValType::i32())],
            outputs: vec![ValType::i64()],
        },
        I::I32Extend8S | I::I32Extend16S => StackType {
            inputs: vec![OpdType::Strict(ValType::i32())],
            outputs: vec![ValType::i32()],
        },
        I::I64Extend8S | I::I64Extend16S | I::I64Extend32S => StackType {
            inputs: vec![OpdType::Strict(ValType::i64())],
            outputs: vec![ValType::i64()],
        },
        I::I64ExtendI32S | I::I64ExtendI32U => StackType {
            inputs: vec![OpdType::Strict(ValType::i32())],
            outputs: vec![ValType::i32()],
        },
        I::I32TruncF32S | I::I32TruncF32U | I::I32TruncSatF32S | I::I32TruncSatF32U => StackType {
            inputs: vec![OpdType::Strict(ValType::f32())],
            outputs: vec![ValType::i32()],
        },
        I::I32TruncF64S | I::I32TruncF64U | I::I32TruncSatF64S | I::I32TruncSatF64U => StackType {
            inputs: vec![OpdType::Strict(ValType::f64())],
            outputs: vec![ValType::i32()],
        },
        I::I64TruncF32S | I::I64TruncF32U | I::I64TruncSatF32S | I::I64TruncSatF32U => StackType {
            inputs: vec![OpdType::Strict(ValType::f32())],
            outputs: vec![ValType::i64()],
        },
        I::I64TruncF64S | I::I64TruncF64U | I::I64TruncSatF64S | I::I64TruncSatF64U => StackType {
            inputs: vec![OpdType::Strict(ValType::f64())],
            outputs: vec![ValType::i64()],
        },
        I::F32ConvertI32S | I::F32ConvertI32U => StackType {
            inputs: vec![OpdType::Strict(ValType::i32())],
            outputs: vec![ValType::f32()],
        },
        I::F32ConvertI64S | I::F32ConvertI64U => StackType {
            inputs: vec![OpdType::Strict(ValType::i64())],
            outputs: vec![ValType::f32()],
        },
        I::F64ConvertI32S | I::F64ConvertI32U => StackType {
            inputs: vec![OpdType::Strict(ValType::i32())],
            outputs: vec![ValType::f64()],
        },
        I::F64ConvertI64S | I::F64ConvertI64U => StackType {
            inputs: vec![OpdType::Strict(ValType::i64())],
            outputs: vec![ValType::f64()],
        },
        I::F32DemoteF64 => StackType {
            inputs: vec![OpdType::Strict(ValType::f64())],
            outputs: vec![ValType::f32()],
        },
        I::F64PromoteF32 => StackType {
            inputs: vec![OpdType::Strict(ValType::f32())],
            outputs: vec![ValType::f64()],
        },
        I::I32ReinterpretF32 => StackType {
            inputs: vec![OpdType::Strict(ValType::f32())],
            outputs: vec![ValType::i32()],
        },
        I::I64ReinterpretF64 => StackType {
            inputs: vec![OpdType::Strict(ValType::f64())],
            outputs: vec![ValType::i64()],
        },
        I::F32ReinterpretI32 => StackType {
            inputs: vec![OpdType::Strict(ValType::i32())],
            outputs: vec![ValType::f32()],
        },
        I::F64ReinterpretI64 => StackType {
            inputs: vec![OpdType::Strict(ValType::i64())],
            outputs: vec![ValType::f64()],
        },
        // Ref Instructions
        I::RefNull(ref ref_type) => StackType {
            inputs: vec![],
            outputs: vec![ValType::RefType(ref_type.clone())],
        },
        I::RefIsNull => StackType {
            inputs: vec![OpdType::AnyOf(ValType::get_ref_types())],
            outputs: vec![ValType::i32()],
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
        I::I8x16Eq
        | I::I8x16Ne
        | I::I8x16LtS
        | I::I8x16LtU
        | I::I8x16GtS
        | I::I8x16GtU
        | I::I8x16LeS
        | I::I8x16LeU
        | I::I8x16GeS
        | I::I8x16GeU
        | I::I16x8Eq
        | I::I16x8Ne
        | I::I16x8LtS
        | I::I16x8LtU
        | I::I16x8GtS
        | I::I16x8GtU
        | I::I16x8LeS
        | I::I16x8LeU
        | I::I16x8GeS
        | I::I16x8GeU
        | I::I32x4Eq
        | I::I32x4Ne
        | I::I32x4LtS
        | I::I32x4LtU
        | I::I32x4GtS
        | I::I32x4GtU
        | I::I32x4LeS
        | I::I32x4LeU
        | I::I32x4GeS
        | I::I32x4GeU
        | I::I64x2Eq
        | I::I64x2Ne
        | I::I64x2LtS
        | I::I64x2GtS
        | I::I64x2LeS
        | I::I64x2GeS
        | I::F32x4Eq
        | I::F32x4Ne
        | I::F32x4Lt
        | I::F32x4Gt
        | I::F32x4Le
        | I::F32x4Ge
        | I::F64x2Eq
        | I::F64x2Ne
        | I::F64x2Lt
        | I::F64x2Gt
        | I::F64x2Le
        | I::F64x2Ge => StackType {
            inputs: vec![
                OpdType::Strict(ValType::v128()),
                OpdType::Strict(ValType::v128()),
            ],
            outputs: vec![ValType::v128()],
        },
        I::I8x16Shl
        | I::I8x16ShrS
        | I::I8x16ShrU
        | I::I16x8Shl
        | I::I16x8ShrS
        | I::I16x8ShrU
        | I::I32x4Shl
        | I::I32x4ShrS
        | I::I32x4ShrU
        | I::I64x2Shl
        | I::I64x2ShrS
        | I::I64x2ShrU => StackType {
            inputs: vec![
                OpdType::Strict(ValType::v128()),
                OpdType::Strict(ValType::i32()),
            ],
            outputs: vec![ValType::v128()],
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
        I::I8x16Add
        | I::I8x16Sub
        | I::I16x8Add
        | I::I16x8Sub
        | I::I32x4Add
        | I::I32x4Sub
        | I::I64x2Add
        | I::I64x2Sub
        | I::F32x4Add
        | I::F32x4Sub
        | I::F32x4Mul
        | I::F32x4Div
        | I::F32x4Min
        | I::F32x4Max
        | I::F32x4Pmin
        | I::F32x4Pmax
        | I::F64x2Add
        | I::F64x2Sub
        | I::F64x2Mul
        | I::F64x2Div
        | I::F64x2Min
        | I::F64x2Max
        | I::F64x2Pmin
        | I::F64x2Pmax
        | I::I8x16MinS
        | I::I8x16MinU
        | I::I8x16MaxS
        | I::I8x16MaxU
        | I::I16x8MinS
        | I::I16x8MinU
        | I::I16x8MaxS
        | I::I16x8MaxU
        | I::I32x4MinS
        | I::I32x4MinU
        | I::I32x4MaxS
        | I::I32x4MaxU
        | I::I8x16AddSatS
        | I::I8x16AddSatU
        | I::I8x16SubSatS
        | I::I8x16SubSatU
        | I::I16x8AddSatS
        | I::I16x8AddSatU
        | I::I16x8SubSatS
        | I::I16x8SubSatU
        | I::I16x8Mul
        | I::I32x4Mul
        | I::I64x2Mul
        | I::I8x16AvgrU
        | I::I16x8AvgrU
        | I::I16x8Q15MulrSatS => StackType {
            inputs: vec![
                OpdType::Strict(ValType::v128()),
                OpdType::Strict(ValType::v128()),
            ],
            outputs: vec![ValType::v128()],
        },
        I::I8x16AllTrue | I::I16x8AllTrue | I::I32x4AllTrue | I::I64x2AllTrue => StackType {
            inputs: vec![
                OpdType::Strict(ValType::v128()),
                OpdType::Strict(ValType::v128()),
            ],
            outputs: vec![ValType::i32()],
        },
        I::I16x8ExtendLowI8x16S
        | I::I16x8ExtendHighI8x16S
        | I::I16x8ExtendLowI8x16U
        | I::I16x8ExtendHighI8x16U
        | I::I32x4ExtendLowI16x8S
        | I::I32x4ExtendHighI16x8S
        | I::I32x4ExtendLowI16x8U
        | I::I32x4ExtendHighI16x8U
        | I::I32x4TruncSatF32x4S
        | I::I32x4TruncSatF32x4U
        | I::I64x2ExtendLowI32x4S
        | I::I64x2ExtendHighI32x4S
        | I::I64x2ExtendLowI32x4U
        | I::I64x2ExtendHighI32x4U
        | I::F32x4ConvertI32x4S
        | I::F32x4ConvertI32x4U
        | I::I32x4TruncSatF64x2SZero
        | I::I32x4TruncSatF64x2UZero
        | I::F64x2ConvertLowI32x4S
        | I::F64x2ConvertLowI32x4U
        | I::F32x4DemoteF64x2Zero
        | I::F64x2PromoteLowF32x4 => StackType {
            inputs: vec![OpdType::Strict(ValType::v128())],
            outputs: vec![ValType::v128()],
        },
        // ishape.narrow_ishape_sx
        I::I16x8NarrowI32x4S
        | I::I16x8NarrowI32x4U
        | I::I8x16NarrowI16x8S
        | I::I8x16NarrowI16x8U => StackType {
            inputs: vec![
                OpdType::Strict(ValType::v128()),
                OpdType::Strict(ValType::v128()),
            ],
            outputs: vec![ValType::v128()],
        },
        // ishape.bitmask
        I::I8x16Bitmask | I::I16x8Bitmask | I::I32x4Bitmask | I::I64x2Bitmask => StackType {
            inputs: vec![OpdType::Strict(ValType::v128())],
            outputs: vec![ValType::i32()],
        },
        // ishape.dot_ishape_s
        I::I32x4DotI16x8S => StackType {
            inputs: vec![
                OpdType::Strict(ValType::v128()),
                OpdType::Strict(ValType::v128()),
            ],
            outputs: vec![ValType::v128()],
        },
        // ishape.extmul_half_ishape_sx
        I::I16x8ExtmulLowI8x16S
        | I::I16x8ExtmulHighI8x16S
        | I::I16x8ExtmulLowI8x16U
        | I::I16x8ExtmulHighI8x16U
        | I::I32x4ExtmulLowI16x8S
        | I::I32x4ExtmulHighI16x8S
        | I::I32x4ExtmulLowI16x8U
        | I::I32x4ExtmulHighI16x8U
        | I::I64x2ExtmulLowI32x4S
        | I::I64x2ExtmulHighI32x4S
        | I::I64x2ExtmulLowI32x4U
        | I::I64x2ExtmulHighI32x4U => StackType {
            inputs: vec![
                OpdType::Strict(ValType::v128()),
                OpdType::Strict(ValType::v128()),
            ],
            outputs: vec![ValType::v128()],
        },
        // ishape.extadd_pairwise_ishape_sx
        I::I16x8ExtaddPairwiseI8x16S
        | I::I16x8ExtaddPairwiseI8x16U
        | I::I32x4ExtaddPairwiseI16x8S
        | I::I32x4ExtaddPairwiseI16x8U => StackType {
            inputs: vec![OpdType::Strict(ValType::v128())],
            outputs: vec![ValType::v128()],
        },
        // Parametric instructions
        I::Drop => StackType {
            inputs: vec![OpdType::Any],
            outputs: vec![],
        },
        I::Select => {
            let mut operand_types = vec![];
            operand_types.append(&mut ValType::get_num_types());
            operand_types.push(ValType::v128());

            StackType {
                inputs: vec![OpdType::AnyOf(operand_types)],
                outputs: vec![ValType::v128()],
            }
        }
        I::SelectVec(val_types_vec) => {
            if val_types_vec.len() != 1 {
                return Err(ValidationError::InvalidSelectVecOperandSequence);
            }

            StackType {
                inputs: vec![
                    OpdType::Strict(val_types_vec[0].clone()),
                    OpdType::Strict(val_types_vec[0].clone()),
                    OpdType::Strict(ValType::i32()),
                ],
                outputs: vec![val_types_vec[0].clone()],
            }
        }
        // Variable Instructions
        I::LocalGet(local_idx) => {
            let local_type = ctx
                .locals
                .get(local_idx.0 .0 as usize)
                .ok_or_else(|| ValidationError::LocalNotFound)?;

            StackType {
                inputs: vec![],
                outputs: vec![local_type.clone()],
            }
        }
        I::LocalSet(local_idx) => {
            let local_type = ctx
                .locals
                .get(local_idx.0 .0 as usize)
                .ok_or_else(|| ValidationError::LocalNotFound)?;

            StackType {
                inputs: vec![OpdType::Strict(local_type.clone())],
                outputs: vec![],
            }
        }
        I::LocalTee(local_idx) => {
            let local_type = ctx
                .locals
                .get(local_idx.0 .0 as usize)
                .ok_or_else(|| ValidationError::LocalNotFound)?;

            StackType {
                inputs: vec![OpdType::Strict(local_type.clone())],
                outputs: vec![local_type.clone()],
            }
        }
        I::GlobalGet(local_idx) => {
            let global_type = ctx
                .globals
                .get(local_idx.0 .0 as usize)
                .ok_or_else(|| ValidationError::GlobalNotFound)?;

            StackType {
                inputs: vec![],
                outputs: vec![global_type.val_type.clone()],
            }
        }
        I::GlobalSet(local_idx) => {
            let global_type = ctx
                .globals
                .get(local_idx.0 .0 as usize)
                .ok_or_else(|| ValidationError::GlobalNotFound)?;

            if global_type.mut_type != MutType::Var {
                return Err(ValidationError::UnableToSetToConstGlobal);
            }

            StackType {
                inputs: vec![OpdType::Strict(global_type.val_type.clone())],
                outputs: vec![],
            }
        }
        // Table Instructions
        I::TableGet(table_idx) => {
            let table = ctx
                .tables
                .get(table_idx.0 .0 as usize)
                .ok_or_else(|| ValidationError::TableNotFound)?;

            StackType {
                inputs: vec![OpdType::Strict(ValType::i32())],
                outputs: vec![ValType::RefType(table.element_ref_type.clone())],
            }
        }
        I::TableSet(table_idx) => {
            let table = ctx
                .tables
                .get(table_idx.0 .0 as usize)
                .ok_or_else(|| ValidationError::TableNotFound)?;

            StackType {
                inputs: vec![
                    OpdType::Strict(ValType::i32()),
                    OpdType::Strict(ValType::RefType(table.element_ref_type.clone())),
                ],
                outputs: vec![],
            }
        }
        I::TableSize(table_idx) => {
            if ctx.tables.get(table_idx.0 .0 as usize).is_none() {
                return Err(ValidationError::TableNotFound);
            }

            StackType {
                inputs: vec![],
                outputs: vec![ValType::i32()],
            }
        }
        I::TableGrow(table_idx) => {
            let table = ctx
                .tables
                .get(table_idx.0 .0 as usize)
                .ok_or_else(|| ValidationError::TableNotFound)?;

            StackType {
                inputs: vec![
                    OpdType::Strict(ValType::RefType(table.element_ref_type.clone())),
                    OpdType::Strict(ValType::i32()),
                ],
                outputs: vec![ValType::i32()],
            }
        }
        I::TableFill(table_idx) => {
            let table = ctx
                .tables
                .get(table_idx.0 .0 as usize)
                .ok_or_else(|| ValidationError::TableNotFound)?;

            StackType {
                inputs: vec![
                    OpdType::Strict(ValType::i32()),
                    OpdType::Strict(ValType::RefType(table.element_ref_type.clone())),
                    OpdType::Strict(ValType::i32()),
                ],
                outputs: vec![],
            }
        }
        I::TableCopy((table_idx_lhs, table_idx_rhs)) => {
            let table_lhs = ctx
                .tables
                .get(table_idx_lhs.0 .0 as usize)
                .ok_or_else(|| ValidationError::TableNotFound)?;

            let table_rhs = ctx
                .tables
                .get(table_idx_rhs.0 .0 as usize)
                .ok_or_else(|| ValidationError::TableNotFound)?;

            if table_lhs.element_ref_type != table_rhs.element_ref_type {
                return Err(ValidationError::UnableToCopyIncosistentTableTypes);
            }

            StackType {
                inputs: vec![
                    OpdType::Strict(ValType::i32()),
                    OpdType::Strict(ValType::i32()),
                    OpdType::Strict(ValType::i32()),
                ],
                outputs: vec![],
            }
        }
        I::TableInit((elem_idx, table_idx)) => {
            let table = ctx
                .tables
                .get(table_idx.0 .0 as usize)
                .ok_or_else(|| ValidationError::TableNotFound)?;
            let table_type = table.element_ref_type.clone();

            let elem = ctx
                .elems
                .get(elem_idx.0 .0 as usize)
                .ok_or_else(|| ValidationError::ElemNotFound)?;

            if *elem != table_type {
                return Err(ValidationError::WrongElemType);
            }

            StackType {
                inputs: vec![
                    OpdType::Strict(ValType::i32()),
                    OpdType::Strict(ValType::i32()),
                    OpdType::Strict(ValType::i32()),
                ],
                outputs: vec![],
            }
        }
        I::ElemDrop(elem_idx) => {
            if ctx.elems.get(elem_idx.0 .0 as usize).is_none() {
                return Err(ValidationError::ElemNotFound);
            }
            StackType {
                inputs: vec![],
                outputs: vec![],
            }
        }
        I::I32Load(mem_arg) => {
            check! {
                memarg 32, ctx, mem_arg
            }

            StackType {
                inputs: vec![OpdType::Strict(ValType::i32())],
                outputs: vec![ValType::i32()],
            }
        }
        I::I64Load(mem_arg) => {
            check! {
                memarg 64, ctx, mem_arg
            }

            StackType {
                inputs: vec![OpdType::Strict(ValType::i32())],
                outputs: vec![ValType::i64()],
            }
        }
        I::F32Load(mem_arg) => {
            check! {
                memarg 32, ctx, mem_arg
            }

            StackType {
                inputs: vec![OpdType::Strict(ValType::i32())],
                outputs: vec![ValType::f32()],
            }
        }
        I::F64Load(mem_arg) => {
            check! {
                memarg 64, ctx, mem_arg
            }

            StackType {
                inputs: vec![OpdType::Strict(ValType::i32())],
                outputs: vec![ValType::f64()],
            }
        }
        I::V128Load(mem_arg) => {
            check! {
                memarg 128, ctx, mem_arg
            }

            StackType {
                inputs: vec![OpdType::Strict(ValType::i32())],
                outputs: vec![ValType::v128()],
            }
        }
        I::I32Load8S(mem_arg) | I::I32Load8U(mem_arg) => {
            check! {
                memarg 8, ctx, mem_arg
            }

            StackType {
                inputs: vec![OpdType::Strict(ValType::i32())],
                outputs: vec![ValType::i32()],
            }
        }
        I::I32Load16S(mem_arg) | I::I32Load16U(mem_arg) => {
            check! {
                memarg 16, ctx, mem_arg
            }

            StackType {
                inputs: vec![OpdType::Strict(ValType::i32())],
                outputs: vec![ValType::i32()],
            }
        }
        I::I64Load8S(mem_arg) | I::I64Load8U(mem_arg) => {
            check! {
                memarg 8, ctx, mem_arg
            }

            StackType {
                inputs: vec![OpdType::Strict(ValType::i32())],
                outputs: vec![ValType::i64()],
            }
        }
        I::I64Load16S(mem_arg) | I::I64Load16U(mem_arg) => {
            check! {
                memarg 16, ctx, mem_arg
            }

            StackType {
                inputs: vec![OpdType::Strict(ValType::i32())],
                outputs: vec![ValType::i64()],
            }
        }
        I::I64Load32S(mem_arg) | I::I64Load32U(mem_arg) => {
            check! {
                memarg 32, ctx, mem_arg
            }

            StackType {
                inputs: vec![OpdType::Strict(ValType::i32())],
                outputs: vec![ValType::i64()],
            }
        }
        I::I32Store(mem_arg) => {
            check! {
                memarg 32, ctx, mem_arg
            }

            StackType {
                inputs: vec![
                    OpdType::Strict(ValType::i32()),
                    OpdType::Strict(ValType::i32()),
                ],
                outputs: vec![],
            }
        }
        I::I64Store(mem_arg) => {
            check! {
                memarg 64, ctx, mem_arg
            }

            StackType {
                inputs: vec![
                    OpdType::Strict(ValType::i32()),
                    OpdType::Strict(ValType::i64()),
                ],
                outputs: vec![],
            }
        }
        I::F32Store(mem_arg) => {
            check! {
                memarg 32, ctx, mem_arg
            }

            StackType {
                inputs: vec![
                    OpdType::Strict(ValType::i32()),
                    OpdType::Strict(ValType::f32()),
                ],
                outputs: vec![],
            }
        }
        I::F64Store(mem_arg) => {
            check! {
                memarg 64, ctx, mem_arg
            }

            StackType {
                inputs: vec![
                    OpdType::Strict(ValType::i32()),
                    OpdType::Strict(ValType::f64()),
                ],
                outputs: vec![],
            }
        }
        I::V128Store(mem_arg) => {
            check! {
                memarg 128, ctx, mem_arg
            }

            StackType {
                inputs: vec![
                    OpdType::Strict(ValType::i32()),
                    OpdType::Strict(ValType::v128()),
                ],
                outputs: vec![],
            }
        }
        I::I32Store8(mem_arg) => {
            check! {
                memarg 8, ctx, mem_arg
            }

            StackType {
                inputs: vec![
                    OpdType::Strict(ValType::i32()),
                    OpdType::Strict(ValType::i32()),
                ],
                outputs: vec![],
            }
        }
        I::I32Store16(mem_arg) => {
            check! {
                memarg 16, ctx, mem_arg
            }

            StackType {
                inputs: vec![
                    OpdType::Strict(ValType::i32()),
                    OpdType::Strict(ValType::i32()),
                ],
                outputs: vec![],
            }
        }
        I::I64Store8(mem_arg) => {
            check! {
                memarg 8, ctx, mem_arg
            }

            StackType {
                inputs: vec![
                    OpdType::Strict(ValType::i32()),
                    OpdType::Strict(ValType::i64()),
                ],
                outputs: vec![],
            }
        }
        I::I64Store16(mem_arg) => {
            check! {
                memarg 16, ctx, mem_arg
            }

            StackType {
                inputs: vec![
                    OpdType::Strict(ValType::i32()),
                    OpdType::Strict(ValType::i64()),
                ],
                outputs: vec![],
            }
        }
        I::I64Store32(mem_arg) => {
            check! {
                memarg 32, ctx, mem_arg
            }

            StackType {
                inputs: vec![
                    OpdType::Strict(ValType::i32()),
                    OpdType::Strict(ValType::i64()),
                ],
                outputs: vec![],
            }
        }
        I::V128Load8x8S(mem_arg) | I::V128Load8x8U(mem_arg) => check! {
            memarg_vec_load ctx, mem_arg, 8, 8
        },
        I::V128Load16x4S(mem_arg) | I::V128Load16x4U(mem_arg) => check! {
            memarg_vec_load ctx, mem_arg, 16, 4
        },
        I::V128Load32x2S(mem_arg) | I::V128Load32x2U(mem_arg) => check! {
            memarg_vec_load ctx, mem_arg, 32, 2
        },
        I::V128Load8Splat(mem_arg) => {
            check! {
                memarg 8, ctx, mem_arg
            }
            StackType {
                inputs: vec![OpdType::Strict(ValType::i32())],
                outputs: vec![ValType::v128()],
            }
        }
        I::V128Load16Splat(mem_arg) => {
            check! {
                memarg 16, ctx, mem_arg
            }
            StackType {
                inputs: vec![OpdType::Strict(ValType::i32())],
                outputs: vec![ValType::v128()],
            }
        }
        I::V128Load32Splat(mem_arg) => {
            check! {
                memarg 32, ctx, mem_arg
            }
            StackType {
                inputs: vec![OpdType::Strict(ValType::i32())],
                outputs: vec![ValType::v128()],
            }
        }
        I::V128Load64Splat(mem_arg) => {
            check! {
                memarg 64, ctx, mem_arg
            }
            StackType {
                inputs: vec![OpdType::Strict(ValType::i32())],
                outputs: vec![ValType::v128()],
            }
        }
        I::V128Load32Zero(mem_arg) => {
            check! {
                memarg 32, ctx, mem_arg
            }
            StackType {
                inputs: vec![OpdType::Strict(ValType::i32())],
                outputs: vec![ValType::v128()],
            }
        }
        I::V128Load64Zero(mem_arg) => {
            check! {
                memarg 64, ctx, mem_arg
            }
            StackType {
                inputs: vec![OpdType::Strict(ValType::i32())],
                outputs: vec![ValType::v128()],
            }
        }
        I::V128Load8Lane((mem_arg, lane_idx)) => check! {
            memarg_vec_load_lane ctx, mem_arg, lane_idx, 8
        },
        I::V128Load16Lane((mem_arg, lane_idx)) => check! {
            memarg_vec_load_lane ctx, mem_arg, lane_idx, 16
        },
        I::V128Load32Lane((mem_arg, lane_idx)) => check! {
            memarg_vec_load_lane ctx, mem_arg, lane_idx, 32
        },
        I::V128Load64Lane((mem_arg, lane_idx)) => check! {
            memarg_vec_load_lane ctx, mem_arg, lane_idx, 64
        },
        I::V128Store8Lane((mem_arg, lane_idx)) => check! {
            memarg_vec_store_lane ctx, mem_arg, lane_idx, 8
        },
        I::V128Store16Lane((mem_arg, lane_idx)) => check! {
            memarg_vec_store_lane ctx, mem_arg, lane_idx, 16
        },
        I::V128Store32Lane((mem_arg, lane_idx)) => check! {
            memarg_vec_store_lane ctx, mem_arg, lane_idx, 32
        },
        I::V128Store64Lane((mem_arg, lane_idx)) => check! {
            memarg_vec_store_lane ctx, mem_arg, lane_idx, 64
        },
        I::MemorySize => {
            if ctx.mems.get(0).is_none() {
                return Err(ValidationError::MemNotFound);
            }

            StackType {
                inputs: vec![],
                outputs: vec![ValType::i32()],
            }
        }
        I::MemoryGrow => {
            if ctx.mems.get(0).is_none() {
                return Err(ValidationError::MemNotFound);
            }

            StackType {
                inputs: vec![OpdType::Strict(ValType::i32())],
                outputs: vec![ValType::i32()],
            }
        }
        I::MemoryFill | I::MemoryCopy => {
            if ctx.mems.get(0).is_none() {
                return Err(ValidationError::MemNotFound);
            }

            StackType {
                inputs: vec![
                    OpdType::Strict(ValType::i32()),
                    OpdType::Strict(ValType::i32()),
                    OpdType::Strict(ValType::i32()),
                ],
                outputs: vec![],
            }
        }
        I::MemoryInit(data_idx) => {
            if ctx.mems.get(0).is_none() {
                return Err(ValidationError::MemNotFound);
            }

            if ctx.datas.get(data_idx.0 .0 as usize).is_none() {
                return Err(ValidationError::DataNotFound);
            }

            StackType {
                inputs: vec![
                    OpdType::Strict(ValType::i32()),
                    OpdType::Strict(ValType::i32()),
                    OpdType::Strict(ValType::i32()),
                ],
                outputs: vec![],
            }
        }
        I::DataDrop(data_idx) => {
            if ctx.datas.get(data_idx.0 .0 as usize).is_none() {
                return Err(ValidationError::DataNotFound);
            }

            StackType {
                inputs: vec![],
                outputs: vec![],
            }
        }
        I::Nop => StackType {
            inputs: vec![],
            outputs: vec![],
        },
        I::Unreachable => unimplemented!(),
        I::Block(_) => unimplemented!(),
        I::Loop(_) => unimplemented!(),
        I::IfElse(_) => unimplemented!(),
        I::Br(_) => unimplemented!(),
        I::BrIf(_) => unimplemented!(),
        I::BrTable(_) => unimplemented!(),
        I::Return => unimplemented!(),
        I::Call(_) => unimplemented!(),
        I::CallIndirect(_) => unimplemented!(),
        // FIXME:
        // I::End => unimplemented!(),
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
