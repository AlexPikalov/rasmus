use syntax::module::InstructionType;

use crate::instances::value::Val;

pub fn test_instruction(
    before_instructions: Vec<InstructionType>,
    instruction: InstructionType,
    expected_val: Val,
) {
    let mut store = crate::instances::store::Store::new();
    let mut stack = crate::instances::stack::Stack::new();

    for ref before in before_instructions {
        crate::execute::execute_instruction(before, &mut stack, &mut store)
            .expect("shouold execute before instruction wihtout errors");
    }

    crate::execute::execute_instruction(&instruction, &mut stack, &mut store)
        .expect("should execute instruction without errors");

    if let Some(val) = stack.pop_value() {
        assert_eq!(val, expected_val);
    } else {
        assert!(false, "stack should contain value");
    }
}
