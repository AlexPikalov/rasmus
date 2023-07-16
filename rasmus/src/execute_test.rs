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

macro_rules! test_instruction_assert {
    ($test_name: ident, $before_instructions: expr, $instruction: expr, $assert: expr) => {
        #[test]
        fn $test_name() {
            let mut store = Store::new();
            let mut stack = Stack::new();

            for ref before in $before_instructions {
                assert!(execute_instruction(before, &mut stack, &mut store).is_ok());
            }

            assert!(execute_instruction(&$instruction, &mut stack, &mut store).is_ok());

            if let Some(val) = stack.pop_value() {
                $assert(val);
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

test_instruction!(
    i32_rem_u,
    vec![
        InstructionType::I32Const(I32Type(7)),
        InstructionType::I32Const(I32Type(2))
    ],
    InstructionType::I32RemU,
    Val::I32(1)
);

test_instruction!(
    i32_rem_s_with_overflow,
    vec![
        InstructionType::I32Const(I32Type(u32::MAX)),
        InstructionType::I32Const(I32Type(2))
    ],
    InstructionType::I32RemS,
    Val::I32(-1i32 as u32)
);

test_instruction!(
    i32_rem_s_no_overflow,
    vec![
        InstructionType::I32Const(I32Type(9)),
        InstructionType::I32Const(I32Type(7))
    ],
    InstructionType::I32RemS,
    Val::I32(2)
);

test_instruction!(
    i64_rem_u,
    vec![
        InstructionType::I64Const(I64Type(7)),
        InstructionType::I64Const(I64Type(2))
    ],
    InstructionType::I64RemU,
    Val::I64(1)
);

test_instruction!(
    i64_rem_s_with_overflow,
    vec![
        InstructionType::I64Const(I64Type(18446744073709551605)),
        InstructionType::I64Const(I64Type(2))
    ],
    InstructionType::I64RemS,
    Val::I64(-1i64 as u64)
);

test_instruction!(
    i64_rem_s_no_overflow,
    vec![
        InstructionType::I64Const(I64Type(9)),
        InstructionType::I64Const(I64Type(7))
    ],
    InstructionType::I64RemS,
    Val::I64(2)
);

test_instruction!(
    i32_and_zero,
    vec![
        InstructionType::I32Const(I32Type(5)),
        InstructionType::I32Const(I32Type(2))
    ],
    InstructionType::I32And,
    Val::I32(0)
);

test_instruction!(
    i32_and_not_zero,
    vec![
        InstructionType::I32Const(I32Type(5)),
        InstructionType::I32Const(I32Type(3))
    ],
    InstructionType::I32And,
    Val::I32(1)
);

test_instruction!(
    i64_and_zero,
    vec![
        InstructionType::I64Const(I64Type(5)),
        InstructionType::I64Const(I64Type(2))
    ],
    InstructionType::I64And,
    Val::I64(0)
);

test_instruction!(
    i64_and_not_zero,
    vec![
        InstructionType::I64Const(I64Type(5)),
        InstructionType::I64Const(I64Type(3))
    ],
    InstructionType::I64And,
    Val::I64(1)
);

test_instruction!(
    i32_or_zero,
    vec![
        InstructionType::I32Const(I32Type(0)),
        InstructionType::I32Const(I32Type(0))
    ],
    InstructionType::I32Or,
    Val::I32(0)
);

test_instruction!(
    i32_or_not_zero,
    vec![
        InstructionType::I32Const(I32Type(5)),
        InstructionType::I32Const(I32Type(3))
    ],
    InstructionType::I32Or,
    Val::I32(7)
);

test_instruction!(
    i64_or_zero,
    vec![
        InstructionType::I64Const(I64Type(0)),
        InstructionType::I64Const(I64Type(0))
    ],
    InstructionType::I64Or,
    Val::I64(0)
);

test_instruction!(
    i64_or_not_zero,
    vec![
        InstructionType::I64Const(I64Type(5)),
        InstructionType::I64Const(I64Type(3))
    ],
    InstructionType::I64Or,
    Val::I64(7)
);

test_instruction!(
    i32_xor_zero,
    vec![
        InstructionType::I32Const(I32Type(2)),
        InstructionType::I32Const(I32Type(2))
    ],
    InstructionType::I32Xor,
    Val::I32(0)
);

test_instruction!(
    i32_xor_not_zero,
    vec![
        InstructionType::I32Const(I32Type(5)),
        InstructionType::I32Const(I32Type(3))
    ],
    InstructionType::I32Xor,
    Val::I32(6)
);

test_instruction!(
    i64_xor_zero,
    vec![
        InstructionType::I64Const(I64Type(2)),
        InstructionType::I64Const(I64Type(2))
    ],
    InstructionType::I64Xor,
    Val::I64(0)
);

test_instruction!(
    i64_xor_not_zero,
    vec![
        InstructionType::I64Const(I64Type(5)),
        InstructionType::I64Const(I64Type(3))
    ],
    InstructionType::I64Xor,
    Val::I64(6)
);

test_instruction!(
    ishl_32_no_overflow,
    vec![
        InstructionType::I32Const(I32Type(10)),
        InstructionType::I32Const(I32Type(3)),
    ],
    InstructionType::I32Shl,
    Val::I32(80)
);

test_instruction!(
    ishl_32_rot_overflow,
    vec![
        InstructionType::I32Const(I32Type(10)),
        InstructionType::I32Const(I32Type(32)),
    ],
    InstructionType::I32Shl,
    Val::I32(10)
);

test_instruction!(
    ishl_32_base_overflow,
    vec![
        InstructionType::I32Const(I32Type(0b10000000000000000000000000000001u32)),
        InstructionType::I32Const(I32Type(1))
    ],
    InstructionType::I32Shl,
    Val::I32(0b10u32)
);

test_instruction!(
    ishl_64_no_overflow,
    vec![
        InstructionType::I64Const(I64Type(10)),
        InstructionType::I64Const(I64Type(3)),
    ],
    InstructionType::I64Shl,
    Val::I64(80)
);

test_instruction!(
    ishl_64_rot_overflow,
    vec![
        InstructionType::I64Const(I64Type(10)),
        InstructionType::I64Const(I64Type(64)),
    ],
    InstructionType::I64Shl,
    Val::I64(10)
);

test_instruction!(
    ishl_64_base_overflow,
    vec![
        InstructionType::I64Const(I64Type(
            0b1000000000000000000000000000000000000000000000000000000000000001u64
        )),
        InstructionType::I64Const(I64Type(1))
    ],
    InstructionType::I64Shl,
    Val::I64(0b10u64)
);

test_instruction!(
    ishr_u_32_not_overflow,
    vec![
        InstructionType::I32Const(I32Type(0b10000000000000000000000000000001u32)),
        InstructionType::I32Const(I32Type(1))
    ],
    InstructionType::I32ShrU,
    Val::I32(0b01000000000000000000000000000000u32)
);

test_instruction!(
    ishr_u_32_rot_overflow,
    vec![
        InstructionType::I32Const(I32Type(0b10000000000000000000000000000001u32)),
        InstructionType::I32Const(I32Type(32))
    ],
    InstructionType::I32ShrU,
    Val::I32(0b10000000000000000000000000000001u32)
);

test_instruction!(
    ishr_u_64_no_overflow,
    vec![
        InstructionType::I64Const(I64Type(
            0b1000000000000000000000000000000000000000000000000000000000000001u64
        )),
        InstructionType::I64Const(I64Type(1))
    ],
    InstructionType::I64ShrU,
    Val::I64(0b0100000000000000000000000000000000000000000000000000000000000000u64)
);

test_instruction!(
    ishr_u_64_rot_overflow,
    vec![
        InstructionType::I64Const(I64Type(
            0b1000000000000000000000000000000000000000000000000000000000000001u64
        )),
        InstructionType::I64Const(I64Type(64))
    ],
    InstructionType::I64ShrU,
    Val::I64(0b1000000000000000000000000000000000000000000000000000000000000001u64)
);

test_instruction!(
    ishr_s_32_no_overflow_zero,
    vec![
        InstructionType::I32Const(I32Type(0b00000000000000000000000000000010u32)),
        InstructionType::I32Const(I32Type(1))
    ],
    InstructionType::I32ShrS,
    Val::I32(0b0000000000000000000000000000001u32)
);

test_instruction!(
    ishr_s_32_no_overflow_one,
    vec![
        InstructionType::I32Const(I32Type(0b10000000000000000000000000001000u32)),
        InstructionType::I32Const(I32Type(3))
    ],
    InstructionType::I32ShrS,
    Val::I32(0b11110000000000000000000000000001u32)
);

test_instruction!(
    ishr_s_32_overflow_rot,
    vec![
        InstructionType::I32Const(I32Type(0b10000000000000000000000000001000u32)),
        InstructionType::I32Const(I32Type(32))
    ],
    InstructionType::I32ShrS,
    Val::I32(0b10000000000000000000000000001000u32)
);

test_instruction!(
    ishr_s_64_no_overflow_zero,
    vec![
        InstructionType::I64Const(I64Type(
            0b0000000000000000000000000000000000000000000000000000000000000010u64
        )),
        InstructionType::I64Const(I64Type(1))
    ],
    InstructionType::I64ShrS,
    Val::I64(0b000000000000000000000000000000000000000000000000000000000000001u64)
);

test_instruction!(
    ishr_s_64_no_overflow_one,
    vec![
        InstructionType::I64Const(I64Type(
            0b1000000000000000000000000000000000000000000000000000000000001000u64
        )),
        InstructionType::I64Const(I64Type(3))
    ],
    InstructionType::I64ShrS,
    Val::I64(0b1111000000000000000000000000000000000000000000000000000000000001u64)
);

test_instruction!(
    ishr_s_64_overflow_rot,
    vec![
        InstructionType::I64Const(I64Type(
            0b1000000000000000000000000000000000000000000000000000000000001000u64
        )),
        InstructionType::I64Const(I64Type(64))
    ],
    InstructionType::I64ShrS,
    Val::I64(0b1000000000000000000000000000000000000000000000000000000000001000u64)
);

test_instruction!(
    irotl_32_no_overflow,
    vec![
        InstructionType::I32Const(I32Type(0b10000000000000000000000000000001u32)),
        InstructionType::I32Const(I32Type(1))
    ],
    InstructionType::I32Rotl,
    Val::I32(3)
);

test_instruction!(
    irotl_32_rot_overflow,
    vec![
        InstructionType::I32Const(I32Type(0b10000000000000000000000000000001u32)),
        InstructionType::I32Const(I32Type(32))
    ],
    InstructionType::I32Rotl,
    Val::I32(0b10000000000000000000000000000001u32)
);

test_instruction!(
    irotl_64_no_overflow,
    vec![
        InstructionType::I64Const(I64Type(
            0b1000000000000000000000000000000000000000000000000000000000000001u64
        )),
        InstructionType::I64Const(I64Type(1))
    ],
    InstructionType::I64Rotl,
    Val::I64(3)
);

test_instruction!(
    irotl_64_rot_overflow,
    vec![
        InstructionType::I64Const(I64Type(
            0b1000000000000000000000000000000000000000000000000000000000000001u64
        )),
        InstructionType::I64Const(I64Type(64))
    ],
    InstructionType::I64Rotl,
    Val::I64(0b1000000000000000000000000000000000000000000000000000000000000001u64)
);

test_instruction!(
    f32_add_no_overflow,
    vec![
        InstructionType::F32Const(F32Type(0.9)),
        InstructionType::F32Const(F32Type(0.1))
    ],
    InstructionType::F32Add,
    Val::F32(1.0)
);

test_instruction!(
    f64_add_no_overflow,
    vec![
        InstructionType::F64Const(F64Type(0.9)),
        InstructionType::F64Const(F64Type(0.1))
    ],
    InstructionType::F64Add,
    Val::F64(1.0)
);

test_instruction!(
    f32_sub_no_overflow,
    vec![
        InstructionType::F32Const(F32Type(0.1)),
        InstructionType::F32Const(F32Type(0.4))
    ],
    InstructionType::F32Sub,
    Val::F32(-0.3)
);

test_instruction!(
    f64_sub_no_overflow,
    vec![
        InstructionType::F64Const(F64Type(0.11)),
        InstructionType::F64Const(F64Type(0.3))
    ],
    InstructionType::F64Sub,
    Val::F64(-0.19)
);

test_instruction!(
    f32_mul_inverse_sign,
    vec![
        InstructionType::F32Const(F32Type(-1.0)),
        InstructionType::F32Const(F32Type(0.3))
    ],
    InstructionType::F32Mul,
    Val::F32(-0.3)
);

test_instruction!(
    f32_mul_no_overflow,
    vec![
        InstructionType::F32Const(F32Type(1.0)),
        InstructionType::F32Const(F32Type(0.3))
    ],
    InstructionType::F32Mul,
    Val::F32(0.3)
);

test_instruction!(
    f64_mul_no_overflow,
    vec![
        InstructionType::F64Const(F64Type(1.0)),
        InstructionType::F64Const(F64Type(0.3))
    ],
    InstructionType::F64Mul,
    Val::F64(0.3)
);

test_instruction!(
    f32_div_no_overflow,
    vec![
        InstructionType::F32Const(F32Type(1.0)),
        InstructionType::F32Const(F32Type(0.2))
    ],
    InstructionType::F32Div,
    Val::F32(5.0)
);

test_instruction!(
    f32_div_neg_infinity,
    vec![
        InstructionType::F32Const(F32Type(1.0)),
        InstructionType::F32Const(F32Type(-0.0))
    ],
    InstructionType::F32Div,
    Val::F32(f32::NEG_INFINITY)
);

test_instruction!(
    f32_div_pos_infinity,
    vec![
        InstructionType::F32Const(F32Type(1.0)),
        InstructionType::F32Const(F32Type(0.0))
    ],
    InstructionType::F32Div,
    Val::F32(f32::INFINITY)
);

test_instruction!(
    f64_div_no_overflow,
    vec![
        InstructionType::F64Const(F64Type(1.0)),
        InstructionType::F64Const(F64Type(0.2))
    ],
    InstructionType::F64Div,
    Val::F64(5.0)
);

test_instruction!(
    f64_div_neg_infinity,
    vec![
        InstructionType::F64Const(F64Type(1.0)),
        InstructionType::F64Const(F64Type(-0.0))
    ],
    InstructionType::F64Div,
    Val::F64(f64::NEG_INFINITY)
);

test_instruction!(
    f64_div_pos_infinity,
    vec![
        InstructionType::F64Const(F64Type(1.0)),
        InstructionType::F64Const(F64Type(0.0))
    ],
    InstructionType::F64Div,
    Val::F64(f64::INFINITY)
);

test_instruction_assert!(
    f32_min_nan_1,
    vec![
        InstructionType::F32Const(F32Type(f32::NAN)),
        InstructionType::F32Const(F32Type(0.0))
    ],
    InstructionType::F32Min,
    |val: Val| {
        match val {
            Val::F32(val_32) => {
                assert!(val_32.is_nan());
            }
            other => {
                assert!(false, "unexpected value type {other:?}, F32 is expected");
            }
        }
    }
);

test_instruction_assert!(
    f32_min_nan_2,
    vec![
        InstructionType::F32Const(F32Type(0.0)),
        InstructionType::F32Const(F32Type(f32::NAN))
    ],
    InstructionType::F32Min,
    |val: Val| {
        match val {
            Val::F32(val_32) => {
                assert!(val_32.is_nan());
            }
            other => {
                assert!(false, "unexpected value type {other:?}, F32 is expected");
            }
        }
    }
);

test_instruction!(
    f32_min,
    vec![
        InstructionType::F32Const(F32Type(0.0)),
        InstructionType::F32Const(F32Type(-0.0))
    ],
    InstructionType::F32Min,
    Val::F32(-0.0)
);

test_instruction_assert!(
    f64_min_nan_1,
    vec![
        InstructionType::F64Const(F64Type(f64::NAN)),
        InstructionType::F64Const(F64Type(0.0))
    ],
    InstructionType::F64Min,
    |val: Val| {
        match val {
            Val::F64(val_64) => {
                assert!(val_64.is_nan());
            }
            other => {
                assert!(false, "unexpected value type {other:?}, F64 is expected");
            }
        }
    }
);

test_instruction_assert!(
    f64_min_nan_2,
    vec![
        InstructionType::F64Const(F64Type(0.0)),
        InstructionType::F64Const(F64Type(f64::NAN))
    ],
    InstructionType::F64Min,
    |val: Val| {
        match val {
            Val::F64(val_64) => {
                assert!(val_64.is_nan());
            }
            other => {
                assert!(false, "unexpected value type {other:?}, F64 is expected");
            }
        }
    }
);

test_instruction!(
    f64_min,
    vec![
        InstructionType::F64Const(F64Type(0.0)),
        InstructionType::F64Const(F64Type(-0.0))
    ],
    InstructionType::F64Min,
    Val::F64(-0.0)
);

test_instruction_assert!(
    f32_max_nan_1,
    vec![
        InstructionType::F32Const(F32Type(f32::NAN)),
        InstructionType::F32Const(F32Type(0.0))
    ],
    InstructionType::F32Max,
    |val: Val| {
        match val {
            Val::F32(val_32) => {
                assert!(val_32.is_nan());
            }
            other => {
                assert!(false, "unexpected value type {other:?}, F32 is expected");
            }
        }
    }
);

test_instruction_assert!(
    f32_max_nan_2,
    vec![
        InstructionType::F32Const(F32Type(0.0)),
        InstructionType::F32Const(F32Type(f32::NAN))
    ],
    InstructionType::F32Max,
    |val: Val| {
        match val {
            Val::F32(val_32) => {
                assert!(val_32.is_nan());
            }
            other => {
                assert!(false, "unexpected value type {other:?}, F32 is expected");
            }
        }
    }
);

test_instruction!(
    f32_max,
    vec![
        InstructionType::F32Const(F32Type(0.0)),
        InstructionType::F32Const(F32Type(-0.0))
    ],
    InstructionType::F32Max,
    Val::F32(0.0)
);

test_instruction_assert!(
    f64_max_nan_1,
    vec![
        InstructionType::F64Const(F64Type(f64::NAN)),
        InstructionType::F64Const(F64Type(0.0))
    ],
    InstructionType::F64Max,
    |val: Val| {
        match val {
            Val::F64(val_64) => {
                assert!(val_64.is_nan());
            }
            other => {
                assert!(false, "unexpected value type {other:?}, F64 is expected");
            }
        }
    }
);

test_instruction_assert!(
    f64_max_nan_2,
    vec![
        InstructionType::F64Const(F64Type(0.0)),
        InstructionType::F64Const(F64Type(f64::NAN))
    ],
    InstructionType::F64Max,
    |val: Val| {
        match val {
            Val::F64(val_64) => {
                assert!(val_64.is_nan());
            }
            other => {
                assert!(false, "unexpected value type {other:?}, F64 is expected");
            }
        }
    }
);

test_instruction!(
    f64_max,
    vec![
        InstructionType::F64Const(F64Type(0.0)),
        InstructionType::F64Const(F64Type(-0.0))
    ],
    InstructionType::F64Max,
    Val::F64(0.0)
);

test_instruction!(
    f32_copy_sign_positive_to_negative,
    vec![
        InstructionType::F32Const(F32Type(-1.0)),
        InstructionType::F32Const(F32Type(2.0))
    ],
    InstructionType::F32Copysign,
    Val::F32(1.0)
);

test_instruction!(
    f64_copy_sign_positive_to_negative,
    vec![
        InstructionType::F64Const(F64Type(-1.0)),
        InstructionType::F64Const(F64Type(2.0))
    ],
    InstructionType::F64Copysign,
    Val::F64(1.0)
);

test_instruction!(
    i32_eqz_true,
    vec![InstructionType::I32Const(I32Type(0))],
    InstructionType::I32Eqz,
    Val::I32(1)
);

test_instruction!(
    i32_eqz_false,
    vec![InstructionType::I32Const(I32Type(1))],
    InstructionType::I32Eqz,
    Val::I32(0)
);

test_instruction!(
    i64_eqz_true,
    vec![InstructionType::I64Const(I64Type(0))],
    InstructionType::I64Eqz,
    Val::I32(1)
);

test_instruction!(
    i64_eqz_false,
    vec![InstructionType::I64Const(I64Type(1))],
    InstructionType::I64Eqz,
    Val::I32(0)
);

test_instruction!(
    i32_eq_true,
    vec![
        InstructionType::I32Const(I32Type(0)),
        InstructionType::I32Const(I32Type(0))
    ],
    InstructionType::I32Eq,
    Val::I32(1)
);

test_instruction!(
    i32_eq_false,
    vec![
        InstructionType::I32Const(I32Type(1)),
        InstructionType::I32Const(I32Type(0))
    ],
    InstructionType::I32Eq,
    Val::I32(0)
);

test_instruction!(
    i64_eq_true,
    vec![
        InstructionType::I64Const(I64Type(0)),
        InstructionType::I64Const(I64Type(0))
    ],
    InstructionType::I64Eq,
    Val::I32(1)
);

test_instruction!(
    i64_eq_false,
    vec![
        InstructionType::I64Const(I64Type(1)),
        InstructionType::I64Const(I64Type(0))
    ],
    InstructionType::I64Eq,
    Val::I32(0)
);

test_instruction!(
    i32_ne_false,
    vec![
        InstructionType::I32Const(I32Type(0)),
        InstructionType::I32Const(I32Type(0))
    ],
    InstructionType::I32Ne,
    Val::I32(0)
);

test_instruction!(
    i32_ne_true,
    vec![
        InstructionType::I32Const(I32Type(1)),
        InstructionType::I32Const(I32Type(0))
    ],
    InstructionType::I32Ne,
    Val::I32(1)
);

test_instruction!(
    i64_ne_false,
    vec![
        InstructionType::I64Const(I64Type(0)),
        InstructionType::I64Const(I64Type(0))
    ],
    InstructionType::I64Ne,
    Val::I32(0)
);

test_instruction!(
    i64_ne_true,
    vec![
        InstructionType::I64Const(I64Type(1)),
        InstructionType::I64Const(I64Type(0))
    ],
    InstructionType::I64Ne,
    Val::I32(1)
);
