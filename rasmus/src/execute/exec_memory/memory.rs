use crate::{
    address::MemAddr,
    execute::exec_const::i32_const,
    instances::{stack::Stack, store::Store},
    result::{RResult, Trap},
};

pub fn memory_size(stack: &mut Stack, store: &mut Store) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get(mem_addr).ok_or(Trap)?;

    let size = mem_inst.size();
    i32_const(&size, stack)
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
