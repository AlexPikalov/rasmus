use super::context::ValidationContext;
use crate::instructions::BlockType;
use crate::types::*;

pub fn is_limit_type_valid(limit: LimitsType, range: U32Type, ctx: ValidationContext) -> bool {
    limit.min <= range
        && limit
            .max
            .map(|max_value| max_value <= range && limit.min <= max_value)
            .unwrap_or(true)
}

pub fn is_block_type_valid(block: BlockType, valid_as: FuncType, ctx: ValidationContext) -> bool {
    match block {
        BlockType::Empty => {
            valid_as
                == FuncType {
                    parameters: vec![],
                    results: vec![],
                }
        }
        BlockType::TypeIndex(type_idx) => ctx
            .types
            .get(type_idx.0 as usize)
            .map(|func_type| *func_type == valid_as)
            .unwrap_or(false),
        BlockType::ValType(val_type) => {
            valid_as
                == FuncType {
                    parameters: vec![],
                    results: vec![val_type],
                }
        }
    }
}

pub fn is_func_type_valid(_func_type: FuncType, _ctx: ValidationContext) -> bool {
    true
}

pub fn is_table_type_valid(table_type: TableType, ctx: ValidationContext) -> bool {
    is_limit_type_valid(table_type.limits, U32Type(u32::MAX), ctx)
}

pub fn is_memory_type_valid(memory_type: MemType, ctx: ValidationContext) -> bool {
    is_limit_type_valid(memory_type.limits, U32Type(2u32.pow(16)), ctx)
}

pub fn is_global_type_valid(_global_type: GlobalType, _ctx: ValidationContext) -> bool {
    true
}

pub fn does_limits_match(lhs: LimitsType, rhs: LimitsType) -> bool {
    lhs.min >= rhs.min
        && (rhs.max.is_none() || rhs.max.zip(lhs.max).map(|(r, l)| l <= r).unwrap_or(false))
}

pub fn does_funcs_match(lhs: FuncType, rhs: FuncType) -> bool {
    lhs == rhs
}

pub fn does_tables_match(lhs: TableType, rhs: TableType) -> bool {
    does_limits_match(lhs.limits, rhs.limits) && lhs.element_ref_type == rhs.element_ref_type
}

pub fn does_memories_match(lhs: MemType, rhs: MemType) -> bool {
    does_limits_match(lhs.limits, rhs.limits)
}

pub fn does_globals_match(lhs: GlobalType, rhs: GlobalType) -> bool {
    lhs == rhs
}
