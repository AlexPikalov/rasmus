use syntax::{
    module::InstructionType,
    types::{F32Type, F64Type, I32Type, I64Type},
};

use crate::{
    execute::{execute_instruction, v128_from_vec},
    instances::{stack::Stack, store::Store, value::Val},
};

macro_rules! test_instruction {
    ($test_name: ident, $instruction: expr, $expected_val: expr) => {
        #[test]
        fn $test_name() {
            let mut store = Store::new();
            let mut stack = Stack::new();

            assert!(execute_instruction(&$instruction, &mut stack, &mut store).is_ok());

            if let Some(val) = stack.pop_value() {
                assert_eq!(val, $expected_val);
            } else {
                assert!(false, "stack should contain value");
            }
        }
    };
    ($test_name: ident, $before_instructions: expr, $instruction: expr, $expected_val: expr) => {
        #[test]
        fn $test_name() {
            let mut store = Store::new();
            let mut stack = Stack::new();

            for ref before in $before_instructions {
                assert!(execute_instruction(before, &mut stack, &mut store).is_ok());
            }

            assert!(execute_instruction(&$instruction, &mut stack, &mut store).is_ok());

            if let Some(val) = stack.pop_value() {
                assert_eq!(val, $expected_val);
            } else {
                assert!(false, "stack should contain value");
            }
        }
    };
}

test_instruction!(
    i32_const,
    InstructionType::I32Const(I32Type(1)),
    Val::I32(1)
);

test_instruction!(
    i64_const,
    InstructionType::I64Const(I64Type(1)),
    Val::I64(1)
);

test_instruction!(
    f32_const,
    InstructionType::F32Const(F32Type(1.0)),
    Val::F32(1.0)
);

test_instruction!(
    f64_const,
    InstructionType::F64Const(F64Type(1.0)),
    Val::F64(1.0)
);

test_instruction!(
    v128_const,
    InstructionType::V128Const(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
    Val::Vec(v128_from_vec(&vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]).unwrap())
);

test_instruction!(
    i32_clz_no_zeros,
    vec![InstructionType::I32Const(I32Type(u32::MAX))],
    InstructionType::I32Clz,
    Val::I32(0)
);

test_instruction!(
    i32_clz_except_first,
    vec![InstructionType::I32Const(I32Type(1))],
    InstructionType::I32Clz,
    Val::I32(u32::BITS - 1)
);

test_instruction!(
    i64_clz_no_zeros,
    vec![InstructionType::I64Const(I64Type(u64::MAX))],
    InstructionType::I64Clz,
    Val::I64(0)
);

test_instruction!(
    i64_clz_except_first,
    vec![InstructionType::I64Const(I64Type(1))],
    InstructionType::I64Clz,
    Val::I64((u64::BITS - 1) as u64)
);

test_instruction!(
    i32_extend8_s_positive,
    vec![InstructionType::I32Const(I32Type(1))],
    InstructionType::I32Extend8S,
    Val::I32(1)
);

test_instruction!(
    i32_extend8_s_negative,
    vec![InstructionType::I32Const(I32Type(4294967290))],
    InstructionType::I32Extend8S,
    Val::I32(-6i8 as u32)
);

test_instruction!(
    i32_extend16_s_positive,
    vec![InstructionType::I32Const(I32Type(1))],
    InstructionType::I32Extend16S,
    Val::I32(1)
);

test_instruction!(
    i32_extend16_s_negative,
    vec![InstructionType::I32Const(I32Type(4294967285))],
    InstructionType::I32Extend16S,
    Val::I32(-11i16 as u32)
);

test_instruction!(
    i64_extend8_s_positive,
    vec![InstructionType::I64Const(I64Type(1))],
    InstructionType::I64Extend8S,
    Val::I64(1)
);

test_instruction!(
    i64_extend8_s_negative,
    vec![InstructionType::I64Const(I64Type(4294967290))],
    InstructionType::I64Extend8S,
    Val::I64(-6i8 as u64)
);

test_instruction!(
    i64_extend16_s_positive,
    vec![InstructionType::I64Const(I64Type(1))],
    InstructionType::I64Extend16S,
    Val::I64(1)
);

test_instruction!(
    i64_extend16_s_negative,
    vec![InstructionType::I64Const(I64Type(4294967285))],
    InstructionType::I64Extend16S,
    Val::I64(-11i16 as u64)
);

test_instruction!(
    i64_extend32_s_positive,
    vec![InstructionType::I64Const(I64Type(1))],
    InstructionType::I64Extend16S,
    Val::I64(1)
);

test_instruction!(
    i64_extend32_s_negative,
    vec![InstructionType::I64Const(I64Type(4294967186))],
    InstructionType::I64Extend16S,
    Val::I64(-110i32 as u64)
);

test_instruction!(
    i32_add_no_overflow,
    vec![
        InstructionType::I32Const(I32Type(1)),
        InstructionType::I32Const(I32Type(0))
    ],
    InstructionType::I32Add,
    Val::I32(1)
);

test_instruction!(
    i32_add_with_overflow,
    vec![
        InstructionType::I32Const(I32Type(2)),
        InstructionType::I32Const(I32Type(u32::MAX)),
    ],
    InstructionType::I32Add,
    Val::I32(1)
);

test_instruction!(
    i64_add_no_overflow,
    vec![
        InstructionType::I64Const(I64Type(0)),
        InstructionType::I64Const(I64Type(1)),
    ],
    InstructionType::I64Add,
    Val::I64(1)
);

test_instruction!(
    i64_add_with_overflow,
    vec![
        InstructionType::I64Const(I64Type(2)),
        InstructionType::I64Const(I64Type(u64::MAX)),
    ],
    InstructionType::I64Add,
    Val::I64(1)
);

test_instruction!(
    i32_sub_no_overflow,
    vec![
        InstructionType::I32Const(I32Type(2)),
        InstructionType::I32Const(I32Type(1)),
    ],
    InstructionType::I32Sub,
    Val::I32(1)
);

test_instruction!(
    i32_sub_with_overflow,
    vec![
        InstructionType::I32Const(I32Type(2)),
        InstructionType::I32Const(I32Type(u32::MAX)),
    ],
    InstructionType::I32Sub,
    Val::I32(3)
);

test_instruction!(
    i64_sub_no_overflow,
    vec![
        InstructionType::I64Const(I64Type(2)),
        InstructionType::I64Const(I64Type(1)),
    ],
    InstructionType::I64Sub,
    Val::I64(1)
);

test_instruction!(
    i64_sub_with_overflow,
    vec![
        InstructionType::I64Const(I64Type(2)),
        InstructionType::I64Const(I64Type(u64::MAX)),
    ],
    InstructionType::I64Sub,
    Val::I64(3)
);

test_instruction!(
    i32_mul_no_overflow,
    vec![
        InstructionType::I32Const(I32Type(4)),
        InstructionType::I32Const(I32Type(3)),
    ],
    InstructionType::I32Mul,
    Val::I32(12)
);

test_instruction!(
    i32_mul_with_overflow,
    vec![
        InstructionType::I32Const(I32Type(3)),
        InstructionType::I32Const(I32Type(u32::MAX / 2)),
    ],
    InstructionType::I32Mul,
    Val::I32(2147483645)
);

test_instruction!(
    i64_mul_no_overflow,
    vec![
        InstructionType::I64Const(I64Type(4)),
        InstructionType::I64Const(I64Type(3)),
    ],
    InstructionType::I64Mul,
    Val::I64(12)
);

test_instruction!(
    i64_mul_with_overflow,
    vec![
        InstructionType::I64Const(I64Type(3)),
        InstructionType::I64Const(I64Type(u64::MAX / 2)),
    ],
    InstructionType::I64Mul,
    Val::I64(9223372036854775805)
);

test_instruction!(
    i32_div_u_no_verflow,
    vec![
        InstructionType::I32Const(I32Type(6)),
        InstructionType::I32Const(I32Type(2)),
    ],
    InstructionType::I32DivU,
    Val::I32(3)
);

test_instruction!(
    i32_div_u_with_verflow,
    vec![
        InstructionType::I32Const(I32Type(-6i32 as u32)),
        InstructionType::I32Const(I32Type(3)),
    ],
    InstructionType::I32DivU,
    Val::I32(1431655763)
);

test_instruction!(
    i32_div_s_no_overflow,
    vec![
        InstructionType::I32Const(I32Type(6)),
        InstructionType::I32Const(I32Type(2)),
    ],
    InstructionType::I32DivS,
    Val::I32(3)
);

test_instruction!(
    i32_div_s_with_overflow,
    vec![
        InstructionType::I32Const(I32Type(2)),
        InstructionType::I32Const(I32Type(4294967295)),
    ],
    InstructionType::I32DivS,
    Val::I32(-2i32 as u32)
);

test_instruction!(
    i64_div_u_no_verflow,
    vec![
        InstructionType::I64Const(I64Type(6)),
        InstructionType::I64Const(I64Type(2)),
    ],
    InstructionType::I64DivU,
    Val::I64(3)
);

test_instruction!(
    i64_div_u_with_verflow,
    vec![
        InstructionType::I64Const(I64Type(-6i64 as u64)),
        InstructionType::I64Const(I64Type(3)),
    ],
    InstructionType::I64DivU,
    Val::I64(6148914691236517203)
);

test_instruction!(
    i64_div_s_no_overflow,
    vec![
        InstructionType::I64Const(I64Type(6)),
        InstructionType::I64Const(I64Type(2)),
    ],
    InstructionType::I64DivS,
    Val::I64(3)
);

test_instruction!(
    i64_div_s_with_overflow,
    vec![
        InstructionType::I64Const(I64Type(18446744073709551605)),
        InstructionType::I64Const(I64Type(3)),
    ],
    InstructionType::I64DivS,
    Val::I64(-3i64 as u64)
);
