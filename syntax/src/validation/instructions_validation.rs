use crate::instructions::{InstructionType, InstructionType as I};
use crate::types::*;

// NOTE: it is used for validating a sequence of instructions
// example: `(i64.const 0) (i32.const 1)` i32.add is invalid
// in a sequence despite each of individual instruction may be valid.

type OpdType = ValType;

impl OpdType {
    pub fn matches(&self, other: OpdType) -> bool {
        *self == other
    }
}

pub struct OperandStack {
    stack: Vec<OpdType>,
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
                .map(|stack_operand| operand_type.matches(stack_operand))
                .unwrap_or(false)
        });

        if is_valid {
            self.stack.append(&mut instruction.inputs);
        }

        is_valid
    }
}

/// Stack types describe how instructions manipulate the operand stack.
pub struct StackType {
    /// Operand types which an instruction pops from a Stack.
    pub inputs: Vec<OpdType>,
    /// Operand types which an instruction pushes back to a Stack as a result.
    pub outputs: Vec<OpdType>,
}

pub fn is_instr_sequence_valid(sequence: Vec<InstructionType>) -> bool {
    let mut operand_stack = OperandStack::with_capacity(sequence.len());

    for instr in sequence {
        let instr_stack_type = get_stack_type_for_instruction(instr);
        let is_valid = operand_stack.validate_and_apply(instr_stack_type);

        if !is_valid {
            return false;
        }
    }

    false
}

pub fn get_stack_type_for_instruction(instruction: InstructionType) -> StackType {
    match instruction {
        // t.const c
        I::I32Const(_) => StackType {
            inputs: vec![],
            outputs: vec![OpdType::NumType(NumType::I32)],
        },
        I::I64Const(_) => StackType {
            inputs: vec![],
            outputs: vec![OpdType::NumType(NumType::I64)],
        },
        I::F32Const(_) => StackType {
            inputs: vec![],
            outputs: vec![OpdType::NumType(NumType::F32)],
        },
        I::F64Const(_) => StackType {
            inputs: vec![],
            outputs: vec![OpdType::NumType(NumType::F64)],
        },

        // t.unop
        I::I32Clz | I::I32Ctz | I::I32Popcnt => StackType {
            inputs: vec![OpdType::NumType(NumType::I32)],
            outputs: vec![OpdType::NumType(NumType::I32)],
        },
        I::I64Clz | I::I64Ctz | I::I64Popcnt => StackType {
            inputs: vec![OpdType::NumType(NumType::I64)],
            outputs: vec![OpdType::NumType(NumType::I64)],
        },
        I::F32Abs
        | I::F32Neg
        | I::F32Ceil
        | I::F32Floor
        | I::F32Trunc
        | I::F32Nearest
        | I::F32Sqrt => StackType {
            inputs: vec![OpdType::NumType(NumType::F32)],
            outputs: vec![OpdType::NumType(NumType::F32)],
        },
        I::F64Abs
        | I::F64Neg
        | I::F64Ceil
        | I::F64Floor
        | I::F64Trunc
        | I::F64Nearest
        | I::F64Sqrt => StackType {
            inputs: vec![OpdType::NumType(NumType::F64)],
            outputs: vec![OpdType::NumType(NumType::F64)],
        },
        // t.binop

        // t.testop
        I::I32Eqz => StackType {
            inputs: vec![OpdType::NumType(NumType::I32)],
            outputs: vec![OpdType::NumType(NumType::I32)],
        },
        I::I64Eqz => StackType {
            inputs: vec![OpdType::NumType(NumType::I64)],
            outputs: vec![OpdType::NumType(NumType::I32)],
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
                OpdType::NumType(NumType::I32),
                OpdType::NumType(NumType::I32),
            ],
            outputs: vec![OpdType::NumType(NumType::I32)],
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
                OpdType::NumType(NumType::I64),
                OpdType::NumType(NumType::I64),
            ],
            outputs: vec![OpdType::NumType(NumType::I64)],
        },
        I::F32Add | I::F32Sub | I::F32Mul | I::F32Div | I::F32Min | I::F32Max | I::F32Copysign => {
            StackType {
                inputs: vec![
                    OpdType::NumType(NumType::F32),
                    OpdType::NumType(NumType::F32),
                ],
                outputs: vec![OpdType::NumType(NumType::F32)],
            }
        }
        I::F64Add | I::F64Sub | I::F64Mul | I::F64Div | I::F64Min | I::F64Max | I::F64Copysign => {
            StackType {
                inputs: vec![
                    OpdType::NumType(NumType::F64),
                    OpdType::NumType(NumType::F64),
                ],
                outputs: vec![OpdType::NumType(NumType::F64)],
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
                OpdType::NumType(NumType::I32),
                OpdType::NumType(NumType::I32),
            ],
            outputs: vec![OpdType::NumType(NumType::I32)],
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
                OpdType::NumType(NumType::I64),
                OpdType::NumType(NumType::I64),
            ],
            outputs: vec![OpdType::NumType(NumType::I32)],
        },
        I::F64Eq | I::F64Ne | I::F64Lt | I::F64Gt | I::F64Le | I::F64Ge => StackType {
            inputs: vec![
                OpdType::NumType(NumType::F64),
                OpdType::NumType(NumType::F64),
            ],
            outputs: vec![OpdType::NumType(NumType::I32)],
        },
        // t.cvtop
        I::I32WrapI64 => StackType {
            inputs: vec![OpdType::NumType(NumType::I32)],
            outputs: vec![OpdType::NumType(NumType::I64)],
        },
        I::I32Extend8S | I::I32Extend16S => StackType {
            inputs: vec![OpdType::NumType(NumType::I32)],
            outputs: vec![OpdType::NumType(NumType::I32)],
        },
        I::I64Extend8S | I::I64Extend16S | I::I64Extend32S => StackType {
            inputs: vec![OpdType::NumType(NumType::I64)],
            outputs: vec![OpdType::NumType(NumType::I64)],
        },
        I::I32TruncF32S | I::I32TruncF32U | I::I32TruncSatF32S | I::I32TruncSatF32U => StackType {
            inputs: vec![OpdType::NumType(NumType::F32)],
            outputs: vec![OpdType::NumType(NumType::I32)],
        },
        I::I32TruncF64S | I::I32TruncF64U | I::I32TruncSatF64S | I::I32TruncSatF64U => StackType {
            inputs: vec![OpdType::NumType(NumType::F64)],
            outputs: vec![OpdType::NumType(NumType::I32)],
        },
        I::I64TruncF32S | I::I64TruncF32U | I::I64TruncSatF32S | I::I64TruncSatF32U => StackType {
            inputs: vec![OpdType::NumType(NumType::F32)],
            outputs: vec![OpdType::NumType(NumType::I64)],
        },
        I::I64TruncF64S | I::I64TruncF64U | I::I64TruncSatF64S | I::I64TruncSatF64U => StackType {
            inputs: vec![OpdType::NumType(NumType::F64)],
            outputs: vec![OpdType::NumType(NumType::I64)],
        },
        I::F32ConvertI32S | I::F32ConvertI32U => StackType {
            inputs: vec![OpdType::NumType(NumType::I32)],
            outputs: vec![OpdType::NumType(NumType::F32)],
        },
        I::F32ConvertI64S | I::F32ConvertI64U => StackType {
            inputs: vec![OpdType::NumType(NumType::I64)],
            outputs: vec![OpdType::NumType(NumType::F32)],
        },
        I::F64ConvertI32S | I::F64ConvertI32U => StackType {
            inputs: vec![OpdType::NumType(NumType::I32)],
            outputs: vec![OpdType::NumType(NumType::F64)],
        },
        I::F64ConvertI64S | I::F64ConvertI64U => StackType {
            inputs: vec![OpdType::NumType(NumType::I64)],
            outputs: vec![OpdType::NumType(NumType::F64)],
        },
        I::F32DemoteF64 => StackType {
            inputs: vec![OpdType::NumType(NumType::F64)],
            outputs: vec![OpdType::NumType(NumType::F32)],
        },
        I::F64PromoteF32 => StackType {
            inputs: vec![OpdType::NumType(NumType::F32)],
            outputs: vec![OpdType::NumType(NumType::F64)],
        },
        I::I32ReinterpretF32 => StackType {
            inputs: vec![OpdType::NumType(NumType::F32)],
            outputs: vec![OpdType::NumType(NumType::I32)],
        },
        I::I64ReinterpretF64 => StackType {
            inputs: vec![OpdType::NumType(NumType::F64)],
            outputs: vec![OpdType::NumType(NumType::I64)],
        },
        I::F32ReinterpretI32 => StackType {
            inputs: vec![OpdType::NumType(NumType::I32)],
            outputs: vec![OpdType::NumType(NumType::F32)],
        },
        I::F64ReinterpretI64 => StackType {
            inputs: vec![OpdType::NumType(NumType::I64)],
            outputs: vec![OpdType::NumType(NumType::F64)],
        },
        // _ => unimplemented!() |
    }
}
