use crate::{
    instances::{
        stack::{Stack, StackEntry},
        value::Val,
    },
    result::RResult,
};

use super::v128_from_vec;

pub fn i32_const(v: &u32, stack: &mut Stack) -> RResult<()> {
    stack.push_entry(StackEntry::Value(Val::I32(*v)));
    Ok(())
}

pub fn i64_const(v: &u64, stack: &mut Stack) -> RResult<()> {
    stack.push_entry(StackEntry::Value(Val::I64(*v)));
    Ok(())
}

pub fn f32_const(v: &f32, stack: &mut Stack) -> RResult<()> {
    stack.push_entry(StackEntry::Value(Val::F32(*v)));
    Ok(())
}

pub fn f64_const(v: &f64, stack: &mut Stack) -> RResult<()> {
    stack.push_entry(StackEntry::Value(Val::F64(*v)));
    Ok(())
}

pub fn v128_const(v: &Vec<u8>, stack: &mut Stack) -> RResult<()> {
    stack.push_entry(StackEntry::Value(Val::Vec(v128_from_vec(v)?)));
    Ok(())
}

#[cfg(test)]
mod test {
    use syntax::{
        module::InstructionType,
        types::{F32Type, F64Type, I32Type, I64Type},
    };

    use crate::{execute::v128_from_vec, instances::value::Val, test_utils::test_instruction};

    #[test]
    fn i32_const() {
        test_instruction(vec![], InstructionType::I32Const(I32Type(1)), Val::I32(1));
    }

    #[test]
    fn i64_const() {
        test_instruction(vec![], InstructionType::I64Const(I64Type(1)), Val::I64(1));
    }

    #[test]
    fn f32_const() {
        test_instruction(
            vec![],
            InstructionType::F32Const(F32Type(1.0)),
            Val::F32(1.0),
        );
    }

    #[test]
    fn f64_const() {
        test_instruction(
            vec![],
            InstructionType::F64Const(F64Type(1.0)),
            Val::F64(1.0),
        );
    }

    #[test]
    fn v128_const() {
        test_instruction(
            vec![],
            InstructionType::V128Const(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
            Val::Vec(v128_from_vec(&vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]).unwrap()),
        );
    }
}
