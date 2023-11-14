use syntax::types::U32Type;

use crate::{
    address::MemAddr,
    execute::exec_const::v128_const,
    instances::{stack::Stack, store::Store},
    result::{RResult, Trap},
};

use super::memory_bytes::{BytesGetter, MemoryBytesGetter};

pub fn v128_load(
    stack: &mut Stack,
    store: &mut Store,
    &(U32Type(offset), U32Type(_align)): &(U32Type, U32Type),
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get(mem_addr).ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;
    let ea = i + offset;
    let bits = 128;

    if (ea + bits / 8) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    v128_const(&MemoryBytesGetter::get_bytes(&mem_inst.data), stack)
}

fn get_mem_addr(stack: &mut Stack) -> RResult<MemAddr> {
    let current_frame = stack.current_frame().ok_or(Trap)?;
    current_frame
        .module
        .borrow()
        .memaddrs
        .get(0)
        .cloned()
        .ok_or(Trap)
}
