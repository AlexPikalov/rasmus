use crate::{
    instances::{stack::Stack, value::Val},
    result::{RResult, Trap},
};

pub fn pop_values_original_order(stack: &mut Stack, m: usize) -> RResult<Vec<Val>> {
    let mut values: Vec<Val> = vec![];

    for _ in 0..m {
        values.push(stack.pop_value().ok_or(Trap)?);
    }

    values.reverse();
    Ok(values)
}
