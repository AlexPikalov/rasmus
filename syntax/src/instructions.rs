#![allow(non_upper_case_globals)]

use super::binary::parse_trait::ParseWithNom;
use super::binary::parser_helpers::{parse, parse_all_to_vec};
use super::types::*;

use nom::{
    bytes::complete::{tag, take},
    IResult as NomResult, Slice,
};

#[derive(Debug, PartialEq)]
pub struct ExpressionType {
    pub instructions: Vec<InstructionType>,
}

impl ExpressionType {
    const OP_CODE_END: Byte = 0x0B;
}

impl ParseWithNom for ExpressionType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, instructions) = parse_all_to_vec(bytes, Self::OP_CODE_END)?;

        Ok((
            bytes,
            ExpressionType {
                instructions: instructions,
            },
        ))
    }
}

#[derive(Debug, PartialEq)]
pub enum BlockType {
    Empty,
    ValType(ValType),
    TypeIndex(S33Type),
}

impl BlockType {
    const OPCODE_EMPTY: Byte = 0x40;
}

impl ParseWithNom for BlockType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        match bytes.get(0) {
            Some(first_byte) => {
                if *first_byte == Self::OPCODE_EMPTY {
                    return Ok((bytes.slice(1..), Self::Empty));
                }

                if let Some(val_type) = ValType::recognize(*first_byte) {
                    return Ok((bytes.slice(1..), Self::ValType(val_type)));
                }

                return S33Type::parse(bytes).map(|(b, v)| (b, Self::TypeIndex(v)));
            }
            None => Err(nom::Err::Failure(nom::error::Error::new(
                bytes,
                nom::error::ErrorKind::Fail,
            ))),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum InstructionType {
    // Control Instructions
    Unreachable,
    Nop,
    Block(BlockInstructionType),
    Loop(LoopInstructionType),
    IfElse(IfElseInstructionType),
    Br(LabelIdx),
    BrIf(LabelIdx),
    BrTable((Vec<LabelIdx>, LabelIdx)),
    Return,
    Call(FuncIdx),
    CallIndirect((TypeIdx, TableIdx)),

    // Reference Instructions
    RefNull(RefType),
    RefIsNull,
    RefFunc(FuncIdx),

    // Parametric Instructions
    Drop,
    Select,
    SelectVec(Vec<ValType>),

    // Variable Instructions
    LocalGet(LocalIdx),
    LocalSet(LocalIdx),
    LocalTee(LocalIdx),
    GlobalGet(GlobalIdx),
    GlobalSet(GlobalIdx),

    // Table Instructions
    TableGet(TableIdx),
    TableSet(TableIdx),
    TableInit((ElemIdx, TableIdx)),
    ElemDrop(ElemIdx),
    TableCopy((TableIdx, TableIdx)),
    TableGrow(TableIdx),
    TableSize(TableIdx),
    TableFill(TableIdx),

    // Memory Instructions
    I32Load((U32Type, U32Type)),
    I64Load((U32Type, U32Type)),
    F32Load((U32Type, U32Type)),
    F64Load((U32Type, U32Type)),
    I32Load8S((U32Type, U32Type)),
    I32Load8U((U32Type, U32Type)),
    I32Load16S((U32Type, U32Type)),
    I32Load16U((U32Type, U32Type)),
    I64Load8S((U32Type, U32Type)),
    I64Load8U((U32Type, U32Type)),
    I64Load16S((U32Type, U32Type)),
    I64Load16U((U32Type, U32Type)),
    I64Load32S((U32Type, U32Type)),
    I64Load32U((U32Type, U32Type)),
    I32Store((U32Type, U32Type)),
    I64Store((U32Type, U32Type)),
    F32Store((U32Type, U32Type)),
    F64Store((U32Type, U32Type)),
    I32Store8((U32Type, U32Type)),
    I32Store16((U32Type, U32Type)),
    I64Store8((U32Type, U32Type)),
    I64Store16((U32Type, U32Type)),
    I64Store32((U32Type, U32Type)),
    MemorySize,
    MemoryGrow,
    MemoryInit(DataIdx),
    DataDrop(DataIdx),
    MemoryCopy,
    MemoryFill,

    // Numeric Instructions
    I32Const(I32Type),
    I64Const(I64Type),
    F32Const(F32Type),
    F64Const(F64Type),
    I32Eqz,
    I32Eq,
    I32Ne,
    I32LtS,
    I32LtU,
    I32GtS,
    I32GtU,
    I32LeS,
    I32LeU,
    I32GeS,
    I32GeU,
    I64Eqz,
    I64Eq,
    I64Ne,
    I64LtS,
    I64LtU,
    I64GtS,
    I64GtU,
    I64LeS,
    I64LeU,
    I64GeS,
    I64GeU,
    F32Eq,
    F32Ne,
    F32Lt,
    F32Gt,
    F32Le,
    F32Ge,
    F64Eq,
    F64Ne,
    F64Lt,
    F64Gt,
    F64Le,
    F64Ge,
    I32Clz,
    I32Ctz,
    I32Popcnt,
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32DivU,
    I32RemS,
    I32RemU,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrS,
    I32ShrU,
    I32Rotl,
    I32Rotr,
    I64Clz,
    I64Ctz,
    I64Popcnt,
    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    I64DivU,
    I64RemS,
    I64RemU,
    I64And,
    I64Or,
    I64Xor,
    I64Shl,
    I64ShrS,
    I64ShrU,
    I64Rotl,
    I64Rotr,
    F32Abs,
    F32Neg,
    F32Ceil,
    F32Floor,
    F32Trunc,
    F32Nearest,
    F32Sqrt,
    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Min,
    F32Max,
    F32Copysign,
    F64Abs,
    F64Neg,
    F64Ceil,
    F64Floor,
    F64Trunc,
    F64Nearest,
    F64Sqrt,
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Min,
    F64Max,
    F64Copysign,
    I32WrapI64,
    I32TruncF32S,
    I32TruncF32U,
    I32TruncF64S,
    I32TruncF64U,
    I64ExtendI32S,
    I64ExtendI32U,
    I64TruncF32S,
    I64TruncF32U,
    I64TruncF64S,
    I64TruncF64U,
    F32ConvertI32S,
    F32ConvertI32U,
    F32ConvertI64S,
    F32ConvertI64U,
    F32DemoteF64,
    F64ConvertI32S,
    F64ConvertI32U,
    F64ConvertI64S,
    F64ConvertI64U,
    F64PromoteF32,
    I32ReinterpretF32,
    I64ReinterpretF64,
    F32ReinterpretI32,
    F64ReinterpretI64,
    I32Extend8S,
    I32Extend16S,
    I64Extend8S,
    I64Extend16S,
    I64Extend32S,
    I32TruncSatF32S,
    I32TruncSatF32U,
    I32TruncSatF64S,
    I32TruncSatF64U,
    I64TruncSatF32S,
    I64TruncSatF32U,
    I64TruncSatF64S,
    I64TruncSatF64U,

    // Vector Instuctions
    V128Load((U32Type, U32Type)),
    V128Load8x8S((U32Type, U32Type)),
    V128Load8x8U((U32Type, U32Type)),
    V128Load16x4S((U32Type, U32Type)),
    V128Load16x4U((U32Type, U32Type)),
    V128Load32x2S((U32Type, U32Type)),
    V128Load32x2U((U32Type, U32Type)),
    V128Load8Splat((U32Type, U32Type)),
    V128Load16Splat((U32Type, U32Type)),
    V128Load32Splat((U32Type, U32Type)),
    V128Load64Splat((U32Type, U32Type)),
    V128Load32Zero((U32Type, U32Type)),
    V128Load64Zero((U32Type, U32Type)),
    V128Store((U32Type, U32Type)),
    V128Load8Lane(((U32Type, U32Type), LaneIdx)),
    V128Load16Lane(((U32Type, U32Type), LaneIdx)),
    V128Load32Lane(((U32Type, U32Type), LaneIdx)),
    V128Load64Lane(((U32Type, U32Type), LaneIdx)),
    V128Store8Lane(((U32Type, U32Type), LaneIdx)),
    V128Store16Lane(((U32Type, U32Type), LaneIdx)),
    V128Store32Lane(((U32Type, U32Type), LaneIdx)),
    V128Store64Lane(((U32Type, U32Type), LaneIdx)),
    // 16 Bytes
    V128Const(Vec<Byte>),
    // 16 LaneIdxs
    I8x16Shuffle(Vec<LaneIdx>),
    I8x16ExtractLaneS(LaneIdx),
    I8x16ExtractLaneU(LaneIdx),
    I8x16ReplaceLane(LaneIdx),
    I16x8ExtractLaneS(LaneIdx),
    I16x8ExtractLaneU(LaneIdx),
    I16x8ReplaceLane(LaneIdx),
    I32x4ExtractLane(LaneIdx),
    I32x4ReplaceLane(LaneIdx),
    I64x2ExtractLane(LaneIdx),
    I64x2ReplaceLane(LaneIdx),
    F32x4ExtractLane(LaneIdx),
    F32x4ReplaceLane(LaneIdx),
    F64x2ExtractLane(LaneIdx),
    F64x2ReplaceLane(LaneIdx),
    I8x16Swizzle,
    I8x16Splat,
    I16x8Splat,
    I32x4Splat,
    I64x2Splat,
    F32x4Splat,
    F64x2Splat,
    I8x16Eq,
    I8x16Ne,
    I8x16LtS,
    I8x16LtU,
    I8x16GtS,
    I8x16GtU,
    I8x16LeS,
    I8x16LeU,
    I8x16GeS,
    I8x16GeU,
    I16x8Eq,
    I16x8Ne,
    I16x8LtS,
    I16x8LtU,
    I16x8GtS,
    I16x8GtU,
    I16x8LeS,
    I16x8LeU,
    I16x8GeS,
    I16x8GeU,
    I32x4Eq,
    I32x4Ne,
    I32x4LtS,
    I32x4LtU,
    I32x4GtS,
    I32x4GtU,
    I32x4LeS,
    I32x4LeU,
    I32x4GeS,
    I32x4GeU,
    I64x2Eq,
    I64x2Ne,
    I64x2LtS,
    I64x2GtS,
    I64x2LeS,
    I64x2GeS,
    F32x4Eq,
    F32x4Ne,
    F32x4Lt,
    F32x4Gt,
    F32x4Le,
    F32x4Ge,
    F64x2Eq,
    F64x2Ne,
    F64x2Lt,
    F64x2Gt,
    F64x2Le,
    F64x2Ge,
    V128Not,
    V128And,
    V128AndNot,
    V128Or,
    V128Xor,
    V128Bitselect,
    V128AnyTrue,
    I8x16Abs,
    I8x16Neg,
    I8x16Popcnt,
    I8x16AllTrue,
    I8x16Bitmask,
    I8x16NarrowI16x8S,
    I8x16NarrowI16x8U,
    I8x16Shl,
    I8x16ShrS,
    I8x16ShrU,
    I8x16Add,
    I8x16AddSatS,
    I8x16AddSatU,
    I8x16Sub,
    I8x16SubSatS,
    I8x16SubSatU,
    I8x16MinS,
    I8x16MinU,
    I8x16MaxS,
    I8x16MaxU,
    I8x16AvgrU,
    I16x8ExtaddPairwiseI8x16S,
    I16x8ExtaddPairwiseI8x16U,
    I16x8Abs,
    I16x8Neg,
    I16x8Q15MulrSatS,
    I16x8AllTrue,
    I16x8Bitmask,
    I16x8NarrowI32x4S,
    I16x8NarrowI32x4U,
    I16x8ExtendLowI8x16S,
    I16x8ExtendHighI8x16S,
    I16x8ExtendLowI8x16U,
    I16x8ExtendHighI8x16U,
    I16x8Shl,
    I16x8ShrS,
    I16x8ShrU,
    I16x8Add,
    I16x8AddSatS,
    I16x8AddSatU,
    I16x8Sub,
    I16x8SubSatS,
    I16x8SubSatU,
    I16x8Mul,
    I16x8MinS,
    I16x8MinU,
    I16x8MaxS,
    I16x8MaxU,
    I16x8AvgrU,
    I16x8ExtmulLowI8x16S,
    I16x8ExtmulHighI8x16S,
    I16x8ExtmulLowI8x16U,
    I16x8ExtmulHighI8x16U,
    I32x4ExtaddPairwiseI16x8S,
    I32x4ExtaddPairwiseI16x8U,
    I32x4Abs,
    I32x4Neg,
    I32x4AllTrue,
    I32x4Bitmask,
    I32x4ExtendLowI16x8S,
    I32x4ExtendHighI16x8S,
    I32x4ExtendLowI16x8U,
    I32x4ExtendHighI16x8U,
    I32x4Shl,
    I32x4ShrS,
    I32x4ShrU,
    I32x4Add,
    I32x4Sub,
    I32x4Mul,
    I32x4MinS,
    I32x4MinU,
    I32x4MaxS,
    I32x4MaxU,
    I32x4DotI16x8S,
    I32x4ExtmulLowI16x8S,
    I32x4ExtmulHighI16x8S,
    I32x4ExtmulLowI16x8U,
    I32x4ExtmulHighI16x8U,
    I64x2Abs,
    I64x2Neg,
    I64x2AllTrue,
    I64x2Bitmask,
    I64x2ExtendLowI32x4S,
    I64x2ExtendHighI32x4S,
    I64x2ExtendLowI32x4U,
    I64x2ExtendHighI32x4U,
    I64x2Shl,
    I64x2ShrS,
    I64x2ShrU,
    I64x2Add,
    I64x2Sub,
    I64x2Mul,
    I64x2ExtmulLowI32x4S,
    I64x2ExtmulHighI32x4S,
    I64x2ExtmulLowI32x4U,
    I64x2ExtmulHighI32x4U,
    F32x4Ceil,
    F32x4Floor,
    F32x4Trunc,
    F32x4Nearest,
    F32x4Abs,
    F32x4Neg,
    F32x4Sqrt,
    F32x4Add,
    F32x4Sub,
    F32x4Mul,
    F32x4Div,
    F32x4Min,
    F32x4Max,
    F32x4Pmin,
    F32x4Pmax,
    F64x2Ceil,
    F64x2Floor,
    F64x2Trunc,
    F64x2Nearest,
    F64x2Abs,
    F64x2Neg,
    F64x2Sqrt,
    F64x2Add,
    F64x2Sub,
    F64x2Mul,
    F64x2Div,
    F64x2Min,
    F64x2Max,
    F64x2Pmin,
    F64x2Pmax,
    I32x4TruncSatF32x4S,
    I32x4TruncSatF32x4U,
    F32x4ConvertI32x4S,
    F32x4ConvertI32x4U,
    I32x4TruncSatF64x2SZero,
    I32x4TruncSatF64x2UZero,
    F64x2ConvertLowI32x4S,
    F64x2ConvertLowI32x4U,
    F32x4DemoteF64x2Zero,
    F64x2PromoteLowF32x4,
}

impl InstructionType {
    // Control Instructions
    const OPCODE_UNREACHABLE: Byte = 0x00;
    const OPCODE_NOP: Byte = 0x01;
    const OPCODE_BLOCK: Byte = 0x02;
    const OPCODE_END: Byte = 0x0B;
    const OPCODE_LOOP: Byte = 0x03;
    const OPCODE_IF_ELSE: Byte = 0x04;
    const OPCODE_ELSE: Byte = 0x05;
    const OPCODE_BR: Byte = 0x0C;
    const OPCODE_BR_IF: Byte = 0x0D;
    const OPCODE_BR_TABLE: Byte = 0x0E;
    const OPCODE_RETURN: Byte = 0x0F;
    const OPCODE_CALL: Byte = 0x10;
    const OPCODE_CALL_INDIRECT: Byte = 0x11;

    // Reference Instructions
    const OPCODE_REF_NULL: Byte = 0xD0;
    const OPCODE_REF_IS_NULL: Byte = 0xD1;
    const OPCODE_REF_FUNC: Byte = 0xD2;

    // Parametric Instructions
    const OPCODE_DROP: Byte = 0x1A;
    const OPCODE_SELECT: Byte = 0x1B;
    const OPCODE_SELECT_VEC: Byte = 0x1C;

    // Variable Instructions
    const OPCODE_LOCAL_GET: Byte = 0x20;
    const OPCODE_LOCAL_SET: Byte = 0x21;
    const OPCODE_LOCAL_TEE: Byte = 0x22;
    const OPCODE_GLOBAL_GET: Byte = 0x23;
    const OPCODE_GLOBAL_SET: Byte = 0x24;

    // Table Instructions
    const OPCODE_TABLE_GET: Byte = 0x25;
    const OPCODE_TABLE_SET: Byte = 0x26;
    const OPCODE_OTHER: Byte = 0xFC;
    const BYTECODE_TABLE_INIT: U32Type = U32Type(12);
    const BYTECODE_TABLE_DROP: U32Type = U32Type(13);
    const BYTECODE_TABLE_COPY: U32Type = U32Type(14);
    const BYTECODE_TABLE_GROW: U32Type = U32Type(15);
    const BYTECODE_TABLE_SIZE: U32Type = U32Type(16);
    const BYTECODE_TABLE_FILL: U32Type = U32Type(17);

    // Memory Instructions
    const OPCODE_I32_LOAD: Byte = 0x28;
    const OPCODE_I64_LOAD: Byte = 0x29;
    const OPCODE_F32_LOAD: Byte = 0x2A;
    const OPCODE_F64_LOAD: Byte = 0x2B;
    const OPCODE_I32_LOAD_8_S: Byte = 0x2C;
    const OPCODE_I32_LOAD_8_U: Byte = 0x2D;
    const OPCODE_I32_LOAD_16_S: Byte = 0x2E;
    const OPCODE_I32_LOAD_16_U: Byte = 0x2F;
    const OPCODE_I64_LOAD_8_S: Byte = 0x30;
    const OPCODE_I64_LOAD_8_U: Byte = 0x31;
    const OPCODE_I64_LOAD_16_S: Byte = 0x32;
    const OPCODE_I64_LOAD_16_U: Byte = 0x33;
    const OPCODE_I64_LOAD_32_S: Byte = 0x34;
    const OPCODE_I64_LOAD_32_U: Byte = 0x35;
    const OPCODE_I32_STORE: Byte = 0x36;
    const OPCODE_I64_STORE: Byte = 0x37;
    const OPCODE_F32_STORE: Byte = 0x38;
    const OPCODE_F64_STORE: Byte = 0x39;
    const OPCODE_I32_STORE_8: Byte = 0x3A;
    const OPCODE_I32_STORE_16: Byte = 0x3B;
    const OPCODE_I64_STORE_8: Byte = 0x3C;
    const OPCODE_I64_STORE_16: Byte = 0x3D;
    const OPCODE_I64_STORE_32: Byte = 0x3E;
    const OPCODE_MEMORY_SIZE: Byte = 0x3F;
    const OPCODE_MEMORY_GROW: Byte = 0x40;
    const BYTECODE_MEMORY_INIT: U32Type = U32Type(8);
    const BYTECODE_DATA_DROP: U32Type = U32Type(9);
    const BYTECODE_MEMORY_COPY: U32Type = U32Type(10);
    const BYTECODE_MEMORY_FILL: U32Type = U32Type(11);

    // Numeric Instructions
    const OPCODE_I32_CONST: Byte = 0x41;
    const OPCODE_I64_CONST: Byte = 0x42;
    const OPCODE_F32_CONST: Byte = 0x43;
    const OPCODE_F64_CONST: Byte = 0x44;
    const OPCODE_I32_EQZ: Byte = 0x45;
    const OPCODE_I32_EQ: Byte = 0x46;
    const OPCODE_I32_NE: Byte = 0x47;
    const OPCODE_I32_LT_S: Byte = 0x48;
    const OPCODE_I32_LT_U: Byte = 0x49;
    const OPCODE_I32_GT_S: Byte = 0x4A;
    const OPCODE_I32_GT_U: Byte = 0x4B;
    const OPCODE_I32_LE_S: Byte = 0x4C;
    const OPCODE_I32_LE_U: Byte = 0x4D;
    const OPCODE_I32_GE_S: Byte = 0x4E;
    const OPCODE_I32_GE_U: Byte = 0x4F;
    const OPCODE_I64_EQZ: Byte = 0x50;
    const OPCODE_I64_EQ: Byte = 0x51;
    const OPCODE_I64_NE: Byte = 0x52;
    const OPCODE_I64_LT_S: Byte = 0x53;
    const OPCODE_I64_LT_U: Byte = 0x54;
    const OPCODE_I64_GT_S: Byte = 0x55;
    const OPCODE_I64_GT_U: Byte = 0x56;
    const OPCODE_I64_LE_S: Byte = 0x57;
    const OPCODE_I64_LE_U: Byte = 0x58;
    const OPCODE_I64_GE_S: Byte = 0x59;
    const OPCODE_I64_GE_U: Byte = 0x5A;
    const OPCODE_F32_EQ: Byte = 0x5B;
    const OPCODE_F32_NE: Byte = 0x5C;
    const OPCODE_F32_LT: Byte = 0x5D;
    const OPCODE_F32_GT: Byte = 0x5E;
    const OPCODE_F32_LE: Byte = 0x5F;
    const OPCODE_F32_GE: Byte = 0x60;
    const OPCODE_F64_EQ: Byte = 0x61;
    const OPCODE_F64_NE: Byte = 0x62;
    const OPCODE_F64_LT: Byte = 0x63;
    const OPCODE_F64_GT: Byte = 0x64;
    const OPCODE_F64_LE: Byte = 0x65;
    const OPCODE_F64_GE: Byte = 0x66;
    const OPCODE_I32_CLZ: Byte = 0x67;
    const OPCODE_I32_CTZ: Byte = 0x68;
    const OPCODE_I32_POPCNT: Byte = 0x69;
    const OPCODE_I32_ADD: Byte = 0x6A;
    const OPCODE_I32_SUB: Byte = 0x6B;
    const OPCODE_I32_MUL: Byte = 0x6C;
    const OPCODE_I32_DIV_S: Byte = 0x6D;
    const OPCODE_I32_DIV_U: Byte = 0x6E;
    const OPCODE_I32_REM_S: Byte = 0x6F;
    const OPCODE_I32_REM_U: Byte = 0x70;
    const OPCODE_I32_AND: Byte = 0x71;
    const OPCODE_I32_OR: Byte = 0x72;
    const OPCODE_I32_XOR: Byte = 0x73;
    const OPCODE_I32_SHL: Byte = 0x74;
    const OPCODE_I32_SHR_S: Byte = 0x75;
    const OPCODE_I32_SHR_U: Byte = 0x76;
    const OPCODE_I32_ROTL: Byte = 0x77;
    const OPCODE_I32_ROTR: Byte = 0x78;
    const OPCODE_I64_CLZ: Byte = 0x79;
    const OPCODE_I64_CTZ: Byte = 0x7A;
    const OPCODE_I64_POPCNT: Byte = 0x7B;
    const OPCODE_I64_ADD: Byte = 0x7C;
    const OPCODE_I64_SUB: Byte = 0x7D;
    const OPCODE_I64_MUL: Byte = 0x7E;
    const OPCODE_I64_DIV_S: Byte = 0x7F;
    const OPCODE_I64_DIV_U: Byte = 0x80;
    const OPCODE_I64_REM_S: Byte = 0x81;
    const OPCODE_I64_REM_U: Byte = 0x82;
    const OPCODE_I64_AND: Byte = 0x83;
    const OPCODE_I64_OR: Byte = 0x84;
    const OPCODE_I64_XOR: Byte = 0x85;
    const OPCODE_I64_SHL: Byte = 0x86;
    const OPCODE_I64_SHR_S: Byte = 0x87;
    const OPCODE_I64_SHR_U: Byte = 0x88;
    const OPCODE_I64_ROTL: Byte = 0x89;
    const OPCODE_I64_ROTR: Byte = 0x8A;
    const OPCODE_F32_ABS: Byte = 0x8B;
    const OPCODE_F32_NEG: Byte = 0x8C;
    const OPCODE_F32_CEIL: Byte = 0x8D;
    const OPCODE_F32_FLOOR: Byte = 0x8E;
    const OPCODE_F32_TRUNC: Byte = 0x8F;
    const OPCODE_F32_NEAREST: Byte = 0x90;
    const OPCODE_F32_SQRT: Byte = 0x91;
    const OPCODE_F32_ADD: Byte = 0x92;
    const OPCODE_F32_SUB: Byte = 0x93;
    const OPCODE_F32_MUL: Byte = 0x94;
    const OPCODE_F32_DIV: Byte = 0x95;
    const OPCODE_F32_MIN: Byte = 0x96;
    const OPCODE_F32_MAX: Byte = 0x97;
    const OPCODE_F32_COPYSIGN: Byte = 0x98;
    const OPCODE_F64_ABS: Byte = 0x99;
    const OPCODE_F64_NEG: Byte = 0x9A;
    const OPCODE_F64_CEIL: Byte = 0x9B;
    const OPCODE_F64_FLOOR: Byte = 0x9C;
    const OPCODE_F64_TRUNC: Byte = 0x9D;
    const OPCODE_F64_NEAREST: Byte = 0x9E;
    const OPCODE_F64_SQRT: Byte = 0x9F;
    const OPCODE_F64_ADD: Byte = 0xA0;
    const OPCODE_F64_SUB: Byte = 0xA1;
    const OPCODE_F64_MUL: Byte = 0xA2;
    const OPCODE_F64_DIV: Byte = 0xA3;
    const OPCODE_F64_MIN: Byte = 0xA4;
    const OPCODE_F64_MAX: Byte = 0xA5;
    const OPCODE_F64_COPYSIGN: Byte = 0xA6;
    const OPCODE_I32_WRAP_I64: Byte = 0xA7;
    const OPCODE_I32_TRUNC_F32_S: Byte = 0xA8;
    const OPCODE_I32_TRUNC_F32_U: Byte = 0xA9;
    const OPCODE_I32_TRUNC_F64_S: Byte = 0xAA;
    const OPCODE_I32_TRUNC_F64_U: Byte = 0xAB;
    const OPCODE_I64_EXTEND_I32_S: Byte = 0xAC;
    const OPCODE_I64_EXTEND_I32_U: Byte = 0xAD;
    const OPCODE_I64_TRUNC_F32_S: Byte = 0xAE;
    const OPCODE_I64_TRUNC_F32_U: Byte = 0xAF;
    const OPCODE_I64_TRUNC_F64_S: Byte = 0xB0;
    const OPCODE_I64_TRUNC_F64_U: Byte = 0xB1;
    const OPCODE_F32_CONVERT_I32_S: Byte = 0xB2;
    const OPCODE_F32_CONVERT_I32_U: Byte = 0xB3;
    const OPCODE_F32_CONVERT_I64_S: Byte = 0xB4;
    const OPCODE_F32_CONVERT_I64_U: Byte = 0xB5;
    const OPCODE_F32_DEMOTE_F64: Byte = 0xB6;
    const OPCODE_F64_CONVERT_I32_S: Byte = 0xB7;
    const OPCODE_F64_CONVERT_I32_U: Byte = 0xB8;
    const OPCODE_F64_CONVERT_I64_S: Byte = 0xB9;
    const OPCODE_F64_CONVERT_I64_U: Byte = 0xBA;
    const OPCODE_F64_PROMOTE_F32: Byte = 0xBB;
    const OPCODE_I32_REINTERPRET_F32: Byte = 0xBC;
    const OPCODE_I64_REINTERPRET_F64: Byte = 0xBD;
    const OPCODE_F32_REINTERPRET_I32: Byte = 0xBE;
    const OPCODE_F64_REINTERPRET_I64: Byte = 0xBF;
    const OPCODE_I32_EXTEND_8_S: Byte = 0xC0;
    const OPCODE_I32_EXTEND_16_S: Byte = 0xC1;
    const OPCODE_I64_EXTEND_8_S: Byte = 0xC2;
    const OPCODE_I64_EXTEND_16_S: Byte = 0xC3;
    const OPCODE_I64_EXTEND_32_S: Byte = 0xC4;
    const BYTE_PREFIX_I32_TRUNC_SAT_F32_S: U32Type = U32Type(0);
    const BYTE_PREFIX_I32_TRUNC_SAT_F32_U: U32Type = U32Type(1);
    const BYTE_PREFIX_I32_TRUNC_SAT_F64_S: U32Type = U32Type(2);
    const BYTE_PREFIX_I32_TRUNC_SAT_F64_U: U32Type = U32Type(3);
    const BYTE_PREFIX_I64_TRUNC_SAT_F32_S: U32Type = U32Type(4);
    const BYTE_PREFIX_I64_TRUNC_SAT_F32_U: U32Type = U32Type(5);
    const BYTE_PREFIX_I64_TRUNC_SAT_F64_S: U32Type = U32Type(6);
    const BYTE_PREFIX_I64_TRUNC_SAT_F64_U: U32Type = U32Type(7);

    // Vector Instuctions
    const OPCODE_VECTOR_INSTRUCTIONS: Byte = 0xFD;
    const BYTE_PREFIX_V128_LOAD: U32Type = U32Type(0);
    const BYTE_PREFIX_V128_LOAD_8x8_S: U32Type = U32Type(1);
    const BYTE_PREFIX_V128_LOAD_8x8_U: U32Type = U32Type(2);
    const BYTE_PREFIX_V128_LOAD_16x4_S: U32Type = U32Type(3);
    const BYTE_PREFIX_V128_LOAD_16x4_U: U32Type = U32Type(4);
    const BYTE_PREFIX_V128_LOAD_32x2_S: U32Type = U32Type(5);
    const BYTE_PREFIX_V128_LOAD_32x2_U: U32Type = U32Type(6);
    const BYTE_PREFIX_V128_LOAD_8_SPLAT: U32Type = U32Type(7);
    const BYTE_PREFIX_V128_LOAD_16_SPLAT: U32Type = U32Type(8);
    const BYTE_PREFIX_V128_LOAD_32_SPLAT: U32Type = U32Type(9);
    const BYTE_PREFIX_V128_LOAD_64_SPLAT: U32Type = U32Type(10);
    const BYTE_PREFIX_V128_LOAD_32_ZERO: U32Type = U32Type(92);
    const BYTE_PREFIX_V128_LOAD_64_ZERO: U32Type = U32Type(93);
    const BYTE_PREFIX_V128_STORE: U32Type = U32Type(11);
    const BYTE_PREFIX_V128_LOAD_8_LANE: U32Type = U32Type(84);
    const BYTE_PREFIX_V128_LOAD_16_LANE: U32Type = U32Type(85);
    const BYTE_PREFIX_V128_LOAD_32_LANE: U32Type = U32Type(86);
    const BYTE_PREFIX_V128_LOAD_64_LANE: U32Type = U32Type(87);
    const BYTE_PREFIX_V128_STORE_8_LANE: U32Type = U32Type(88);
    const BYTE_PREFIX_V128_STORE_16_LANE: U32Type = U32Type(89);
    const BYTE_PREFIX_V128_STORE_32_LANE: U32Type = U32Type(90);
    const BYTE_PREFIX_V128_STORE_64_LANE: U32Type = U32Type(91);
    const BYTE_PREFIX_V128_CONST: U32Type = U32Type(12);
    const BYTE_PREFIX_I8x16_SHUFFLE: U32Type = U32Type(13);
    const BYTE_PREFIX_I8x16_EXTRACT_LANE_S: U32Type = U32Type(21);
    const BYTE_PREFIX_I8x16_EXTRACT_LANE_U: U32Type = U32Type(22);
    const BYTE_PREFIX_I8x16_REPLACE_LANE: U32Type = U32Type(23);
    const BYTE_PREFIX_I16x8_EXTRACT_LANE_S: U32Type = U32Type(24);
    const BYTE_PREFIX_I16x8_EXTRACT_LANE_U: U32Type = U32Type(25);
    const BYTE_PREFIX_I16x8_REPLACE_LANE: U32Type = U32Type(26);
    const BYTE_PREFIX_I32x4_EXTRACT_LANE: U32Type = U32Type(27);
    const BYTE_PREFIX_I32x4_REPLACE_LANE: U32Type = U32Type(28);
    const BYTE_PREFIX_I64x2_EXTRACT_LANE: U32Type = U32Type(29);
    const BYTE_PREFIX_I64x2_REPLACE_LANE: U32Type = U32Type(30);
    const BYTE_PREFIX_F32x4_EXTRACT_LANE: U32Type = U32Type(31);
    const BYTE_PREFIX_F32x4_REPLACE_LANE: U32Type = U32Type(32);
    const BYTE_PREFIX_F64x2_EXTRACT_LANE: U32Type = U32Type(33);
    const BYTE_PREFIX_F64x2_REPLACE_LANE: U32Type = U32Type(34);
    const BYTE_PREFIX_I8x16_SWIZZLE: U32Type = U32Type(14);
    const BYTE_PREFIX_I8x16_SPLAT: U32Type = U32Type(15);
    const BYTE_PREFIX_I16x8_SPLAT: U32Type = U32Type(16);
    const BYTE_PREFIX_I32x4_SPLAT: U32Type = U32Type(17);
    const BYTE_PREFIX_I64x2_SPLAT: U32Type = U32Type(18);
    const BYTE_PREFIX_F32x4_SPLAT: U32Type = U32Type(19);
    const BYTE_PREFIX_F64x2_SPLAT: U32Type = U32Type(20);
    const BYTE_PREFIX_I8x16_EQ: U32Type = U32Type(35);
    const BYTE_PREFIX_I8x16_NE: U32Type = U32Type(36);
    const BYTE_PREFIX_I8x16_LT_S: U32Type = U32Type(37);
    const BYTE_PREFIX_I8x16_LT_U: U32Type = U32Type(38);
    const BYTE_PREFIX_I8x16_GT_S: U32Type = U32Type(39);
    const BYTE_PREFIX_I8x16_GT_U: U32Type = U32Type(40);
    const BYTE_PREFIX_I8x16_LE_S: U32Type = U32Type(41);
    const BYTE_PREFIX_I8x16_LE_U: U32Type = U32Type(42);
    const BYTE_PREFIX_I8x16_GE_S: U32Type = U32Type(43);
    const BYTE_PREFIX_I8x16_GE_U: U32Type = U32Type(44);
    const BYTE_PREFIX_I16x8_EQ: U32Type = U32Type(45);
    const BYTE_PREFIX_I16x8_NE: U32Type = U32Type(46);
    const BYTE_PREFIX_I16x8_LT_S: U32Type = U32Type(47);
    const BYTE_PREFIX_I16x8_LT_U: U32Type = U32Type(48);
    const BYTE_PREFIX_I16x8_GT_S: U32Type = U32Type(49);
    const BYTE_PREFIX_I16x8_GT_U: U32Type = U32Type(50);
    const BYTE_PREFIX_I16x8_LE_S: U32Type = U32Type(51);
    const BYTE_PREFIX_I16x8_LE_U: U32Type = U32Type(52);
    const BYTE_PREFIX_I16x8_GE_S: U32Type = U32Type(53);
    const BYTE_PREFIX_I16x8_GE_U: U32Type = U32Type(54);
    const BYTE_PREFIX_I32x4_EQ: U32Type = U32Type(55);
    const BYTE_PREFIX_I32x4_NE: U32Type = U32Type(56);
    const BYTE_PREFIX_I32x4_LT_S: U32Type = U32Type(57);
    const BYTE_PREFIX_I32x4_LT_U: U32Type = U32Type(58);
    const BYTE_PREFIX_I32x4_GT_S: U32Type = U32Type(59);
    const BYTE_PREFIX_I32x4_GT_U: U32Type = U32Type(60);
    const BYTE_PREFIX_I32x4_LE_S: U32Type = U32Type(61);
    const BYTE_PREFIX_I32x4_LE_U: U32Type = U32Type(62);
    const BYTE_PREFIX_I32x4_GE_S: U32Type = U32Type(63);
    const BYTE_PREFIX_I32x4_GE_U: U32Type = U32Type(64);
    const BYTE_PREFIX_I64x2_EQ: U32Type = U32Type(214);
    const BYTE_PREFIX_I64x2_NE: U32Type = U32Type(215);
    const BYTE_PREFIX_I64x2_LT_S: U32Type = U32Type(216);
    const BYTE_PREFIX_I64x2_GT_S: U32Type = U32Type(217);
    const BYTE_PREFIX_I64x2_LE_S: U32Type = U32Type(218);
    const BYTE_PREFIX_I64x2_GE_S: U32Type = U32Type(219);
    const BYTE_PREFIX_F32x4_EQ: U32Type = U32Type(65);
    const BYTE_PREFIX_F32x4_NE: U32Type = U32Type(66);
    const BYTE_PREFIX_F32x4_LT: U32Type = U32Type(67);
    const BYTE_PREFIX_F32x4_GT: U32Type = U32Type(68);
    const BYTE_PREFIX_F32x4_LE: U32Type = U32Type(69);
    const BYTE_PREFIX_F32x4_GE: U32Type = U32Type(70);
    const BYTE_PREFIX_F64x2_EQ: U32Type = U32Type(71);
    const BYTE_PREFIX_F64x2_NE: U32Type = U32Type(72);
    const BYTE_PREFIX_F64x2_LT: U32Type = U32Type(73);
    const BYTE_PREFIX_F64x2_GT: U32Type = U32Type(74);
    const BYTE_PREFIX_F64x2_LE: U32Type = U32Type(75);
    const BYTE_PREFIX_F64x2_GE: U32Type = U32Type(76);
    const BYTE_PREFIX_V128_NOT: U32Type = U32Type(77);
    const BYTE_PREFIX_V128_AND: U32Type = U32Type(78);
    const BYTE_PREFIX_V128_ANDNOT: U32Type = U32Type(79);
    const BYTE_PREFIX_V128_OR: U32Type = U32Type(80);
    const BYTE_PREFIX_V128_XOR: U32Type = U32Type(81);
    const BYTE_PREFIX_V128_BITSELECT: U32Type = U32Type(82);
    const BYTE_PREFIX_V128_ANYTRUE: U32Type = U32Type(83);
    const BYTE_PREFIX_I8x16_ABS: U32Type = U32Type(96);
    const BYTE_PREFIX_I8x16_NEG: U32Type = U32Type(97);
    const BYTE_PREFIX_I8x16_POPCNT: U32Type = U32Type(98);
    const BYTE_PREFIX_I8x16_ALL_TRUE: U32Type = U32Type(99);
    const BYTE_PREFIX_I8x16_BITMASK: U32Type = U32Type(100);
    const BYTE_PREFIX_I8x16_NARROW_I16x8_S: U32Type = U32Type(101);
    const BYTE_PREFIX_I8x16_NARROW_I16x8_U: U32Type = U32Type(102);
    const BYTE_PREFIX_I8x16_SHL: U32Type = U32Type(107);
    const BYTE_PREFIX_I8x16_SHR_S: U32Type = U32Type(108);
    const BYTE_PREFIX_I8x16_SHR_U: U32Type = U32Type(109);
    const BYTE_PREFIX_I8x16_ADD: U32Type = U32Type(110);
    const BYTE_PREFIX_I8x16_ADD_SAT_S: U32Type = U32Type(111);
    const BYTE_PREFIX_I8x16_ADD_SAT_U: U32Type = U32Type(112);
    const BYTE_PREFIX_I8x16_SUB: U32Type = U32Type(113);
    const BYTE_PREFIX_I8x16_SUB_SAT_S: U32Type = U32Type(114);
    const BYTE_PREFIX_I8x16_SUB_SAT_U: U32Type = U32Type(115);
    const BYTE_PREFIX_I8x16_MIN_S: U32Type = U32Type(118);
    const BYTE_PREFIX_I8x16_MIN_U: U32Type = U32Type(119);
    const BYTE_PREFIX_I8x16_MAX_S: U32Type = U32Type(120);
    const BYTE_PREFIX_I8x16_MAX_U: U32Type = U32Type(121);
    const BYTE_PREFIX_I8x16_AVGR_U: U32Type = U32Type(123);
    const BYTE_PREFIX_I16x8_EXTADD_PAIRWISE_I8x16_S: U32Type = U32Type(124);
    const BYTE_PREFIX_I16x8_EXTADD_PAIRWISE_I8x16_U: U32Type = U32Type(125);
    const BYTE_PREFIX_I16x8_ABS: U32Type = U32Type(128);
    const BYTE_PREFIX_I16x8_NEG: U32Type = U32Type(129);
    const BYTE_PREFIX_I16x8_Q15MULR_SAT_S: U32Type = U32Type(130);
    const BYTE_PREFIX_I16x8_ALL_TRUE: U32Type = U32Type(131);
    const BYTE_PREFIX_I16x8_BITMASK: U32Type = U32Type(132);
    const BYTE_PREFIX_I16x8_NARROW_I32x4_S: U32Type = U32Type(133);
    const BYTE_PREFIX_I16x8_NARROW_I32x4_U: U32Type = U32Type(134);
    const BYTE_PREFIX_I16x8_EXTEND_LOW_I8x16_S: U32Type = U32Type(135);
    const BYTE_PREFIX_I16x8_EXTEND_HIGH_I8x16_S: U32Type = U32Type(136);
    const BYTE_PREFIX_I16x8_EXTEND_LOW_I8x16_U: U32Type = U32Type(137);
    const BYTE_PREFIX_I16x8_EXTEND_HIGH_I8x16_U: U32Type = U32Type(138);
    const BYTE_PREFIX_I16x8_SHL: U32Type = U32Type(139);
    const BYTE_PREFIX_I16x8_SHR_S: U32Type = U32Type(140);
    const BYTE_PREFIX_I16x8_SHR_U: U32Type = U32Type(141);
    const BYTE_PREFIX_I16x8_ADD: U32Type = U32Type(142);
    const BYTE_PREFIX_I16x8_ADD_SAT_S: U32Type = U32Type(143);
    const BYTE_PREFIX_I16x8_ADD_SAT_U: U32Type = U32Type(144);
    const BYTE_PREFIX_I16x8_SUB: U32Type = U32Type(145);
    const BYTE_PREFIX_I16x8_SUB_SAT_S: U32Type = U32Type(146);
    const BYTE_PREFIX_I16x8_SUB_SAT_U: U32Type = U32Type(147);
    const BYTE_PREFIX_I16x8_MUL: U32Type = U32Type(149);
    const BYTE_PREFIX_I16x8_MIN_S: U32Type = U32Type(150);
    const BYTE_PREFIX_I16x8_MIN_U: U32Type = U32Type(151);
    const BYTE_PREFIX_I16x8_MAX_S: U32Type = U32Type(152);
    const BYTE_PREFIX_I16x8_MAX_U: U32Type = U32Type(153);
    const BYTE_PREFIX_I16x8_AVGR_U: U32Type = U32Type(155);
    const BYTE_PREFIX_I16x8_EXTMUL_LOW_I8x16_S: U32Type = U32Type(156);
    const BYTE_PREFIX_I16x8_EXTMUL_HIGH_I8x16_S: U32Type = U32Type(157);
    const BYTE_PREFIX_I16x8_EXTMUL_LOW_I8x16_U: U32Type = U32Type(158);
    const BYTE_PREFIX_I16x8_EXTMUL_HIGH_I8x16_U: U32Type = U32Type(159);
    const BYTE_PREFIX_I32x4_EXTADD_PAIRWISE_I16x8_S: U32Type = U32Type(126);
    const BYTE_PREFIX_I32x4_EXTADD_PAIRWISE_I16x8_U: U32Type = U32Type(127);
    const BYTE_PREFIX_I32x4_ABS: U32Type = U32Type(160);
    const BYTE_PREFIX_I32x4_NEG: U32Type = U32Type(161);
    const BYTE_PREFIX_I32x4_ALL_TRUE: U32Type = U32Type(163);
    const BYTE_PREFIX_I32x4_BITMASK: U32Type = U32Type(164);
    const BYTE_PREFIX_I32x4_EXTEND_LOW_I16x8_S: U32Type = U32Type(167);
    const BYTE_PREFIX_I32x4_EXTEND_HIGH_I16x8_S: U32Type = U32Type(168);
    const BYTE_PREFIX_I32x4_EXTEND_LOW_I16x8_U: U32Type = U32Type(169);
    const BYTE_PREFIX_I32x4_EXTEND_HIGH_I16x8_U: U32Type = U32Type(170);
    const BYTE_PREFIX_I32x4_SHL: U32Type = U32Type(171);
    const BYTE_PREFIX_I32x4_SHR_S: U32Type = U32Type(172);
    const BYTE_PREFIX_I32x4_SHR_U: U32Type = U32Type(173);
    const BYTE_PREFIX_I32x4_ADD: U32Type = U32Type(174);
    const BYTE_PREFIX_I32x4_SUB: U32Type = U32Type(177);
    const BYTE_PREFIX_I32x4_MUL: U32Type = U32Type(181);
    const BYTE_PREFIX_I32x4_MIN_S: U32Type = U32Type(182);
    const BYTE_PREFIX_I32x4_MIN_U: U32Type = U32Type(183);
    const BYTE_PREFIX_I32x4_MAX_S: U32Type = U32Type(184);
    const BYTE_PREFIX_I32x4_MAX_U: U32Type = U32Type(185);
    const BYTE_PREFIX_I32x4_DOT_I16x8_S: U32Type = U32Type(186);
    const BYTE_PREFIX_I32x4_EXTMUL_LOW_I16x8_S: U32Type = U32Type(188);
    const BYTE_PREFIX_I32x4_EXTMUL_HIGH_I16x8_S: U32Type = U32Type(189);
    const BYTE_PREFIX_I32x4_EXTMUL_LOW_I16x8_U: U32Type = U32Type(190);
    const BYTE_PREFIX_I32x4_EXTMUL_HIGH_I16x8_U: U32Type = U32Type(191);
    const BYTE_PREFIX_I64x2_ABS: U32Type = U32Type(192);
    const BYTE_PREFIX_I64x2_NEG: U32Type = U32Type(193);
    const BYTE_PREFIX_I64x2_ALL_TRUE: U32Type = U32Type(195);
    const BYTE_PREFIX_I64x2_BITMASK: U32Type = U32Type(196);
    const BYTE_PREFIX_I64x2_EXTEND_LOW_I32x4_S: U32Type = U32Type(199);
    const BYTE_PREFIX_I64x2_EXTEND_HIGH_I32x4_S: U32Type = U32Type(200);
    const BYTE_PREFIX_I64x2_EXTEND_LOW_I32x4_U: U32Type = U32Type(201);
    const BYTE_PREFIX_I64x2_EXTEND_HIGH_I32x4_U: U32Type = U32Type(202);
    const BYTE_PREFIX_I64x2_SHL: U32Type = U32Type(203);
    const BYTE_PREFIX_I64x2_SHR_S: U32Type = U32Type(204);
    const BYTE_PREFIX_I64x2_SHR_U: U32Type = U32Type(205);
    const BYTE_PREFIX_I64x2_ADD: U32Type = U32Type(206);
    const BYTE_PREFIX_I64x2_SUB: U32Type = U32Type(209);
    const BYTE_PREFIX_I64x2_MUL: U32Type = U32Type(213);
    const BYTE_PREFIX_I64x2_EXTMUL_LOW_I32x4_S: U32Type = U32Type(220);
    const BYTE_PREFIX_I64x2_EXTMUL_HIGH_I32x4_S: U32Type = U32Type(221);
    const BYTE_PREFIX_I64x2_EXTMUL_LOW_I32x4_U: U32Type = U32Type(222);
    const BYTE_PREFIX_I64x2_EXTMUL_HIGH_I32x4_U: U32Type = U32Type(223);
    const BYTE_PREFIX_F32x4_CEIL: U32Type = U32Type(103);
    const BYTE_PREFIX_F32x4_FLOOR: U32Type = U32Type(104);
    const BYTE_PREFIX_F32x4_TRUNC: U32Type = U32Type(105);
    const BYTE_PREFIX_F32x4_NEAREST: U32Type = U32Type(106);
    const BYTE_PREFIX_F32x4_ABS: U32Type = U32Type(224);
    const BYTE_PREFIX_F32x4_NEG: U32Type = U32Type(225);
    const BYTE_PREFIX_F32x4_SQRT: U32Type = U32Type(227);
    const BYTE_PREFIX_F32x4_ADD: U32Type = U32Type(228);
    const BYTE_PREFIX_F32x4_SUB: U32Type = U32Type(229);
    const BYTE_PREFIX_F32x4_MUL: U32Type = U32Type(230);
    const BYTE_PREFIX_F32x4_DIV: U32Type = U32Type(231);
    const BYTE_PREFIX_F32x4_MIN: U32Type = U32Type(232);
    const BYTE_PREFIX_F32x4_MAX: U32Type = U32Type(233);
    const BYTE_PREFIX_F32x4_PMIN: U32Type = U32Type(234);
    const BYTE_PREFIX_F32x4_PMAX: U32Type = U32Type(235);
    const BYTE_PREFIX_F64x2_CEIL: U32Type = U32Type(116);
    const BYTE_PREFIX_F64x2_FLOOR: U32Type = U32Type(117);
    const BYTE_PREFIX_F64x2_TRUNC: U32Type = U32Type(122);
    const BYTE_PREFIX_F64x2_NEAREST: U32Type = U32Type(148);
    const BYTE_PREFIX_F64x2_ABS: U32Type = U32Type(236);
    const BYTE_PREFIX_F64x2_NEG: U32Type = U32Type(237);
    const BYTE_PREFIX_F64x2_SQRT: U32Type = U32Type(239);
    const BYTE_PREFIX_F64x2_ADD: U32Type = U32Type(240);
    const BYTE_PREFIX_F64x2_SUB: U32Type = U32Type(241);
    const BYTE_PREFIX_F64x2_MUL: U32Type = U32Type(242);
    const BYTE_PREFIX_F64x2_DIV: U32Type = U32Type(243);
    const BYTE_PREFIX_F64x2_MIN: U32Type = U32Type(244);
    const BYTE_PREFIX_F64x2_MAX: U32Type = U32Type(245);
    const BYTE_PREFIX_F64x2_PMIN: U32Type = U32Type(246);
    const BYTE_PREFIX_F64x2_PMAX: U32Type = U32Type(247);
    const BYTE_PREFIX_I32x4_TRUNC_SAT_F32x4_S: U32Type = U32Type(248);
    const BYTE_PREFIX_I32x4_TRUNC_SAT_F32x4_U: U32Type = U32Type(249);
    const BYTE_PREFIX_F32x4_CONVERT_I32x4_S: U32Type = U32Type(250);
    const BYTE_PREFIX_F32x4_CONVERT_I32x4_U: U32Type = U32Type(251);
    const BYTE_PREFIX_I32x4_TRUNC_SAT_F64x2_S_ZERO: U32Type = U32Type(252);
    const BYTE_PREFIX_I32x4_TRUNC_SAT_F64x2_U_ZERO: U32Type = U32Type(253);
    const BYTE_PREFIX_F64x2_CONVERT_LOW_I32x4_S: U32Type = U32Type(254);
    const BYTE_PREFIX_F64x2_CONVERT_LOW_I32x4_U: U32Type = U32Type(255);
    const BYTE_PREFIX_F32x4_DEMOTE_F64x2_ZERO: U32Type = U32Type(94);
    const BYTE_PREFIX_F64x2_PROMOTE_LOW_F32x4: U32Type = U32Type(95);

    fn parse_other(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, bytecode) = U32Type::parse(bytes)?;

        match bytecode {
            Self::BYTE_PREFIX_I32_TRUNC_SAT_F32_S => Ok((bytes, Self::I32TruncSatF32S)),
            Self::BYTE_PREFIX_I32_TRUNC_SAT_F32_U => Ok((bytes, Self::I32TruncSatF32U)),
            Self::BYTE_PREFIX_I32_TRUNC_SAT_F64_S => Ok((bytes, Self::I32TruncSatF64S)),
            Self::BYTE_PREFIX_I32_TRUNC_SAT_F64_U => Ok((bytes, Self::I32TruncSatF64U)),
            Self::BYTE_PREFIX_I64_TRUNC_SAT_F32_S => Ok((bytes, Self::I64TruncSatF32S)),
            Self::BYTE_PREFIX_I64_TRUNC_SAT_F32_U => Ok((bytes, Self::I64TruncSatF32U)),
            Self::BYTE_PREFIX_I64_TRUNC_SAT_F64_S => Ok((bytes, Self::I64TruncSatF64S)),
            Self::BYTE_PREFIX_I64_TRUNC_SAT_F64_U => Ok((bytes, Self::I64TruncSatF64U)),

            Self::BYTECODE_MEMORY_INIT => parse(bytes).map(|(b, v)| (b, Self::MemoryInit(v))),
            Self::BYTECODE_DATA_DROP => parse(bytes).map(|(b, v)| (b, Self::DataDrop(v))),
            Self::BYTECODE_MEMORY_COPY => {
                Ok((tag(&[0x00, 0x00])(bytes).map(|r| r.0)?, Self::MemoryCopy))
            }
            Self::BYTECODE_MEMORY_FILL => Ok((tag(&[0x00])(bytes).map(|r| r.0)?, Self::MemoryFill)),
            Self::BYTECODE_TABLE_INIT => parse(bytes).map(|(b, v)| (b, Self::TableInit(v))),
            Self::BYTECODE_TABLE_DROP => parse(bytes).map(|(b, v)| (b, Self::ElemDrop(v))),
            Self::BYTECODE_TABLE_COPY => parse(bytes).map(|(b, v)| (b, Self::TableCopy(v))),
            Self::BYTECODE_TABLE_GROW => parse(bytes).map(|(b, v)| (b, Self::TableGrow(v))),
            Self::BYTECODE_TABLE_SIZE => parse(bytes).map(|(b, v)| (b, Self::TableSize(v))),
            Self::BYTECODE_TABLE_FILL => parse(bytes).map(|(b, v)| (b, Self::TableFill(v))),
            _ => Err(nom::Err::Failure(nom::error::Error::new(
                bytes,
                nom::error::ErrorKind::Fail,
            ))),
        }
    }

    fn parse_vector_instruction(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, byteprefix) = U32Type::parse(bytes)?;

        match byteprefix {
            Self::BYTE_PREFIX_V128_LOAD => parse(bytes).map(|(b, v)| (b, Self::V128Load(v))),
            Self::BYTE_PREFIX_V128_LOAD_8x8_S => {
                parse(bytes).map(|(b, v)| (b, Self::V128Load8x8S(v)))
            }
            Self::BYTE_PREFIX_V128_LOAD_8x8_U => {
                parse(bytes).map(|(b, v)| (b, Self::V128Load8x8U(v)))
            }
            Self::BYTE_PREFIX_V128_LOAD_16x4_S => {
                parse(bytes).map(|(b, v)| (b, Self::V128Load16x4S(v)))
            }
            Self::BYTE_PREFIX_V128_LOAD_16x4_U => {
                parse(bytes).map(|(b, v)| (b, Self::V128Load16x4U(v)))
            }
            Self::BYTE_PREFIX_V128_LOAD_32x2_S => {
                parse(bytes).map(|(b, v)| (b, Self::V128Load32x2S(v)))
            }
            Self::BYTE_PREFIX_V128_LOAD_32x2_U => {
                parse(bytes).map(|(b, v)| (b, Self::V128Load32x2U(v)))
            }
            Self::BYTE_PREFIX_V128_LOAD_8_SPLAT => {
                parse(bytes).map(|(b, v)| (b, Self::V128Load8Splat(v)))
            }
            Self::BYTE_PREFIX_V128_LOAD_16_SPLAT => {
                parse(bytes).map(|(b, v)| (b, Self::V128Load16Splat(v)))
            }
            Self::BYTE_PREFIX_V128_LOAD_32_SPLAT => {
                parse(bytes).map(|(b, v)| (b, Self::V128Load32Splat(v)))
            }
            Self::BYTE_PREFIX_V128_LOAD_64_SPLAT => {
                parse(bytes).map(|(b, v)| (b, Self::V128Load64Splat(v)))
            }
            Self::BYTE_PREFIX_V128_LOAD_32_ZERO => {
                parse(bytes).map(|(b, v)| (b, Self::V128Load32Zero(v)))
            }
            Self::BYTE_PREFIX_V128_LOAD_64_ZERO => {
                parse(bytes).map(|(b, v)| (b, Self::V128Load64Zero(v)))
            }
            Self::BYTE_PREFIX_V128_STORE => parse(bytes).map(|(b, v)| (b, Self::V128Store(v))),
            Self::BYTE_PREFIX_V128_LOAD_8_LANE => {
                parse(bytes).map(|(b, v)| (b, Self::V128Load8Lane(v)))
            }
            Self::BYTE_PREFIX_V128_LOAD_16_LANE => {
                parse(bytes).map(|(b, v)| (b, Self::V128Load16Lane(v)))
            }
            Self::BYTE_PREFIX_V128_LOAD_32_LANE => {
                parse(bytes).map(|(b, v)| (b, Self::V128Load32Lane(v)))
            }
            Self::BYTE_PREFIX_V128_LOAD_64_LANE => {
                parse(bytes).map(|(b, v)| (b, Self::V128Load64Lane(v)))
            }
            Self::BYTE_PREFIX_V128_STORE_8_LANE => {
                parse(bytes).map(|(b, v)| (b, Self::V128Store8Lane(v)))
            }
            Self::BYTE_PREFIX_V128_STORE_16_LANE => {
                parse(bytes).map(|(b, v)| (b, Self::V128Store16Lane(v)))
            }
            Self::BYTE_PREFIX_V128_STORE_32_LANE => {
                parse(bytes).map(|(b, v)| (b, Self::V128Store32Lane(v)))
            }
            Self::BYTE_PREFIX_V128_STORE_64_LANE => {
                parse(bytes).map(|(b, v)| (b, Self::V128Store64Lane(v)))
            }
            Self::BYTE_PREFIX_V128_CONST => parse(bytes).map(|(b, v)| (b, Self::V128Const(v))),
            Self::BYTE_PREFIX_I8x16_SHUFFLE => {
                parse(bytes).map(|(b, v)| (b, Self::I8x16Shuffle(v)))
            }
            Self::BYTE_PREFIX_I8x16_EXTRACT_LANE_S => {
                parse(bytes).map(|(b, v)| (b, Self::I8x16ExtractLaneS(v)))
            }
            Self::BYTE_PREFIX_I8x16_EXTRACT_LANE_U => {
                parse(bytes).map(|(b, v)| (b, Self::I8x16ExtractLaneU(v)))
            }
            Self::BYTE_PREFIX_I8x16_REPLACE_LANE => {
                parse(bytes).map(|(b, v)| (b, Self::I8x16ReplaceLane(v)))
            }
            Self::BYTE_PREFIX_I16x8_EXTRACT_LANE_S => {
                parse(bytes).map(|(b, v)| (b, Self::I16x8ExtractLaneS(v)))
            }
            Self::BYTE_PREFIX_I16x8_EXTRACT_LANE_U => {
                parse(bytes).map(|(b, v)| (b, Self::I16x8ExtractLaneU(v)))
            }
            Self::BYTE_PREFIX_I16x8_REPLACE_LANE => {
                parse(bytes).map(|(b, v)| (b, Self::I16x8ReplaceLane(v)))
            }
            Self::BYTE_PREFIX_I32x4_EXTRACT_LANE => {
                parse(bytes).map(|(b, v)| (b, Self::I32x4ExtractLane(v)))
            }
            Self::BYTE_PREFIX_I32x4_REPLACE_LANE => {
                parse(bytes).map(|(b, v)| (b, Self::I32x4ReplaceLane(v)))
            }
            Self::BYTE_PREFIX_I64x2_EXTRACT_LANE => {
                parse(bytes).map(|(b, v)| (b, Self::I64x2ExtractLane(v)))
            }
            Self::BYTE_PREFIX_I64x2_REPLACE_LANE => {
                parse(bytes).map(|(b, v)| (b, Self::I64x2ReplaceLane(v)))
            }
            Self::BYTE_PREFIX_F32x4_EXTRACT_LANE => {
                parse(bytes).map(|(b, v)| (b, Self::F32x4ExtractLane(v)))
            }
            Self::BYTE_PREFIX_F32x4_REPLACE_LANE => {
                parse(bytes).map(|(b, v)| (b, Self::F32x4ReplaceLane(v)))
            }
            Self::BYTE_PREFIX_F64x2_EXTRACT_LANE => {
                parse(bytes).map(|(b, v)| (b, Self::F64x2ExtractLane(v)))
            }
            Self::BYTE_PREFIX_F64x2_REPLACE_LANE => {
                parse(bytes).map(|(b, v)| (b, Self::F64x2ReplaceLane(v)))
            }
            Self::BYTE_PREFIX_I8x16_SWIZZLE => Ok((bytes, Self::I8x16Swizzle)),
            Self::BYTE_PREFIX_I8x16_SPLAT => Ok((bytes, Self::I8x16Splat)),
            Self::BYTE_PREFIX_I16x8_SPLAT => Ok((bytes, Self::I16x8Splat)),
            Self::BYTE_PREFIX_I32x4_SPLAT => Ok((bytes, Self::I32x4Splat)),
            Self::BYTE_PREFIX_I64x2_SPLAT => Ok((bytes, Self::I64x2Splat)),
            Self::BYTE_PREFIX_F32x4_SPLAT => Ok((bytes, Self::F32x4Splat)),
            Self::BYTE_PREFIX_F64x2_SPLAT => Ok((bytes, Self::F64x2Splat)),
            Self::BYTE_PREFIX_I8x16_EQ => Ok((bytes, Self::I8x16Eq)),
            Self::BYTE_PREFIX_I8x16_NE => Ok((bytes, Self::I8x16Ne)),
            Self::BYTE_PREFIX_I8x16_LT_S => Ok((bytes, Self::I8x16LtS)),
            Self::BYTE_PREFIX_I8x16_LT_U => Ok((bytes, Self::I8x16LtU)),
            Self::BYTE_PREFIX_I8x16_GT_S => Ok((bytes, Self::I8x16GtS)),
            Self::BYTE_PREFIX_I8x16_GT_U => Ok((bytes, Self::I8x16GtU)),
            Self::BYTE_PREFIX_I8x16_LE_S => Ok((bytes, Self::I8x16LeS)),
            Self::BYTE_PREFIX_I8x16_LE_U => Ok((bytes, Self::I8x16LeU)),
            Self::BYTE_PREFIX_I8x16_GE_S => Ok((bytes, Self::I8x16GeS)),
            Self::BYTE_PREFIX_I8x16_GE_U => Ok((bytes, Self::I8x16GeU)),
            Self::BYTE_PREFIX_I16x8_EQ => Ok((bytes, Self::I16x8Eq)),
            Self::BYTE_PREFIX_I16x8_NE => Ok((bytes, Self::I16x8Ne)),
            Self::BYTE_PREFIX_I16x8_LT_S => Ok((bytes, Self::I16x8LtS)),
            Self::BYTE_PREFIX_I16x8_LT_U => Ok((bytes, Self::I16x8LtU)),
            Self::BYTE_PREFIX_I16x8_GT_S => Ok((bytes, Self::I16x8GtS)),
            Self::BYTE_PREFIX_I16x8_GT_U => Ok((bytes, Self::I16x8GtU)),
            Self::BYTE_PREFIX_I16x8_LE_S => Ok((bytes, Self::I16x8LeS)),
            Self::BYTE_PREFIX_I16x8_LE_U => Ok((bytes, Self::I16x8LeU)),
            Self::BYTE_PREFIX_I16x8_GE_S => Ok((bytes, Self::I16x8GeS)),
            Self::BYTE_PREFIX_I16x8_GE_U => Ok((bytes, Self::I16x8GeU)),
            Self::BYTE_PREFIX_I32x4_EQ => Ok((bytes, Self::I32x4Eq)),
            Self::BYTE_PREFIX_I32x4_NE => Ok((bytes, Self::I32x4Ne)),
            Self::BYTE_PREFIX_I32x4_LT_S => Ok((bytes, Self::I32x4LtS)),
            Self::BYTE_PREFIX_I32x4_LT_U => Ok((bytes, Self::I32x4LtU)),
            Self::BYTE_PREFIX_I32x4_GT_S => Ok((bytes, Self::I32x4GtS)),
            Self::BYTE_PREFIX_I32x4_GT_U => Ok((bytes, Self::I32x4GtU)),
            Self::BYTE_PREFIX_I32x4_LE_S => Ok((bytes, Self::I32x4LeS)),
            Self::BYTE_PREFIX_I32x4_LE_U => Ok((bytes, Self::I32x4LeU)),
            Self::BYTE_PREFIX_I32x4_GE_S => Ok((bytes, Self::I32x4GeS)),
            Self::BYTE_PREFIX_I32x4_GE_U => Ok((bytes, Self::I32x4GeU)),
            Self::BYTE_PREFIX_I64x2_EQ => Ok((bytes, Self::I64x2Eq)),
            Self::BYTE_PREFIX_I64x2_NE => Ok((bytes, Self::I64x2Ne)),
            Self::BYTE_PREFIX_I64x2_LT_S => Ok((bytes, Self::I64x2LtS)),
            Self::BYTE_PREFIX_I64x2_GT_S => Ok((bytes, Self::I64x2GtS)),
            Self::BYTE_PREFIX_I64x2_LE_S => Ok((bytes, Self::I64x2LeS)),
            Self::BYTE_PREFIX_I64x2_GE_S => Ok((bytes, Self::I64x2GeS)),
            Self::BYTE_PREFIX_F32x4_EQ => Ok((bytes, Self::F32x4Eq)),
            Self::BYTE_PREFIX_F32x4_NE => Ok((bytes, Self::F32x4Ne)),
            Self::BYTE_PREFIX_F32x4_LT => Ok((bytes, Self::F32x4Lt)),
            Self::BYTE_PREFIX_F32x4_GT => Ok((bytes, Self::F32x4Gt)),
            Self::BYTE_PREFIX_F32x4_LE => Ok((bytes, Self::F32x4Le)),
            Self::BYTE_PREFIX_F32x4_GE => Ok((bytes, Self::F32x4Ge)),
            Self::BYTE_PREFIX_F64x2_EQ => Ok((bytes, Self::F64x2Eq)),
            Self::BYTE_PREFIX_F64x2_NE => Ok((bytes, Self::F64x2Ne)),
            Self::BYTE_PREFIX_F64x2_LT => Ok((bytes, Self::F64x2Lt)),
            Self::BYTE_PREFIX_F64x2_GT => Ok((bytes, Self::F64x2Gt)),
            Self::BYTE_PREFIX_F64x2_LE => Ok((bytes, Self::F64x2Le)),
            Self::BYTE_PREFIX_F64x2_GE => Ok((bytes, Self::F64x2Ge)),
            Self::BYTE_PREFIX_V128_NOT => Ok((bytes, Self::V128Not)),
            Self::BYTE_PREFIX_V128_AND => Ok((bytes, Self::V128And)),
            Self::BYTE_PREFIX_V128_ANDNOT => Ok((bytes, Self::V128AndNot)),
            Self::BYTE_PREFIX_V128_OR => Ok((bytes, Self::V128Or)),
            Self::BYTE_PREFIX_V128_XOR => Ok((bytes, Self::V128Xor)),
            Self::BYTE_PREFIX_V128_BITSELECT => Ok((bytes, Self::V128Bitselect)),
            Self::BYTE_PREFIX_V128_ANYTRUE => Ok((bytes, Self::V128Bitselect)),
            Self::BYTE_PREFIX_I8x16_ABS => Ok((bytes, Self::I8x16Abs)),
            Self::BYTE_PREFIX_I8x16_NEG => Ok((bytes, Self::I8x16Neg)),
            Self::BYTE_PREFIX_I8x16_POPCNT => Ok((bytes, Self::I8x16Popcnt)),
            Self::BYTE_PREFIX_I8x16_ALL_TRUE => Ok((bytes, Self::I8x16AllTrue)),
            Self::BYTE_PREFIX_I8x16_BITMASK => Ok((bytes, Self::I8x16Bitmask)),
            Self::BYTE_PREFIX_I8x16_NARROW_I16x8_S => Ok((bytes, Self::I8x16NarrowI16x8S)),
            Self::BYTE_PREFIX_I8x16_NARROW_I16x8_U => Ok((bytes, Self::I8x16NarrowI16x8U)),
            Self::BYTE_PREFIX_I8x16_SHL => Ok((bytes, Self::I8x16Shl)),
            Self::BYTE_PREFIX_I8x16_SHR_S => Ok((bytes, Self::I8x16ShrS)),
            Self::BYTE_PREFIX_I8x16_SHR_U => Ok((bytes, Self::I8x16ShrU)),
            Self::BYTE_PREFIX_I8x16_ADD => Ok((bytes, Self::I8x16Add)),
            Self::BYTE_PREFIX_I8x16_ADD_SAT_S => Ok((bytes, Self::I8x16AddSatS)),
            Self::BYTE_PREFIX_I8x16_ADD_SAT_U => Ok((bytes, Self::I8x16AddSatU)),
            Self::BYTE_PREFIX_I8x16_SUB => Ok((bytes, Self::I8x16Sub)),
            Self::BYTE_PREFIX_I8x16_SUB_SAT_S => Ok((bytes, Self::I8x16SubSatS)),
            Self::BYTE_PREFIX_I8x16_SUB_SAT_U => Ok((bytes, Self::I8x16SubSatU)),
            Self::BYTE_PREFIX_I8x16_MIN_S => Ok((bytes, Self::I8x16MinS)),
            Self::BYTE_PREFIX_I8x16_MIN_U => Ok((bytes, Self::I8x16MinU)),
            Self::BYTE_PREFIX_I8x16_MAX_S => Ok((bytes, Self::I8x16MaxS)),
            Self::BYTE_PREFIX_I8x16_MAX_U => Ok((bytes, Self::I8x16MaxU)),
            Self::BYTE_PREFIX_I8x16_AVGR_U => Ok((bytes, Self::I8x16AvgrU)),
            Self::BYTE_PREFIX_I16x8_EXTADD_PAIRWISE_I8x16_S => {
                Ok((bytes, Self::I16x8ExtaddPairwiseI8x16S))
            }
            Self::BYTE_PREFIX_I16x8_EXTADD_PAIRWISE_I8x16_U => {
                Ok((bytes, Self::I16x8ExtaddPairwiseI8x16U))
            }
            Self::BYTE_PREFIX_I16x8_ABS => Ok((bytes, Self::I16x8Abs)),
            Self::BYTE_PREFIX_I16x8_NEG => Ok((bytes, Self::I16x8Neg)),
            Self::BYTE_PREFIX_I16x8_Q15MULR_SAT_S => Ok((bytes, Self::I16x8Q15MulrSatS)),
            Self::BYTE_PREFIX_I16x8_ALL_TRUE => Ok((bytes, Self::I8x16AllTrue)),
            Self::BYTE_PREFIX_I16x8_BITMASK => Ok((bytes, Self::I8x16Bitmask)),
            Self::BYTE_PREFIX_I16x8_NARROW_I32x4_S => Ok((bytes, Self::I16x8NarrowI32x4S)),
            Self::BYTE_PREFIX_I16x8_NARROW_I32x4_U => Ok((bytes, Self::I16x8NarrowI32x4U)),
            Self::BYTE_PREFIX_I16x8_EXTEND_LOW_I8x16_S => Ok((bytes, Self::I16x8ExtendLowI8x16S)),
            Self::BYTE_PREFIX_I16x8_EXTEND_HIGH_I8x16_S => Ok((bytes, Self::I16x8ExtendHighI8x16S)),
            Self::BYTE_PREFIX_I16x8_EXTEND_LOW_I8x16_U => Ok((bytes, Self::I16x8ExtendLowI8x16U)),
            Self::BYTE_PREFIX_I16x8_EXTEND_HIGH_I8x16_U => Ok((bytes, Self::I16x8ExtendHighI8x16U)),
            Self::BYTE_PREFIX_I16x8_SHL => Ok((bytes, Self::I16x8Shl)),
            Self::BYTE_PREFIX_I16x8_SHR_S => Ok((bytes, Self::I16x8ShrS)),
            Self::BYTE_PREFIX_I16x8_SHR_U => Ok((bytes, Self::I16x8ShrU)),
            Self::BYTE_PREFIX_I16x8_ADD => Ok((bytes, Self::I16x8Add)),
            Self::BYTE_PREFIX_I16x8_ADD_SAT_S => Ok((bytes, Self::I16x8AddSatS)),
            Self::BYTE_PREFIX_I16x8_ADD_SAT_U => Ok((bytes, Self::I16x8AddSatU)),
            Self::BYTE_PREFIX_I16x8_SUB => Ok((bytes, Self::I16x8Sub)),
            Self::BYTE_PREFIX_I16x8_SUB_SAT_S => Ok((bytes, Self::I16x8SubSatS)),
            Self::BYTE_PREFIX_I16x8_SUB_SAT_U => Ok((bytes, Self::I16x8SubSatU)),
            Self::BYTE_PREFIX_I16x8_MUL => Ok((bytes, Self::I16x8Mul)),
            Self::BYTE_PREFIX_I16x8_MIN_S => Ok((bytes, Self::I16x8MinS)),
            Self::BYTE_PREFIX_I16x8_MIN_U => Ok((bytes, Self::I16x8MinU)),
            Self::BYTE_PREFIX_I16x8_MAX_S => Ok((bytes, Self::I16x8MaxS)),
            Self::BYTE_PREFIX_I16x8_MAX_U => Ok((bytes, Self::I16x8MaxU)),
            Self::BYTE_PREFIX_I16x8_AVGR_U => Ok((bytes, Self::I16x8AvgrU)),
            Self::BYTE_PREFIX_I16x8_EXTMUL_LOW_I8x16_S => Ok((bytes, Self::I16x8ExtmulLowI8x16S)),
            Self::BYTE_PREFIX_I16x8_EXTMUL_HIGH_I8x16_S => Ok((bytes, Self::I16x8ExtmulHighI8x16S)),
            Self::BYTE_PREFIX_I16x8_EXTMUL_LOW_I8x16_U => Ok((bytes, Self::I16x8ExtmulLowI8x16U)),
            Self::BYTE_PREFIX_I16x8_EXTMUL_HIGH_I8x16_U => Ok((bytes, Self::I16x8ExtmulHighI8x16U)),
            Self::BYTE_PREFIX_I32x4_EXTADD_PAIRWISE_I16x8_S => {
                Ok((bytes, Self::I32x4ExtaddPairwiseI16x8S))
            }
            Self::BYTE_PREFIX_I32x4_EXTADD_PAIRWISE_I16x8_U => {
                Ok((bytes, Self::I32x4ExtaddPairwiseI16x8U))
            }
            Self::BYTE_PREFIX_I32x4_ABS => Ok((bytes, Self::I32x4Abs)),
            Self::BYTE_PREFIX_I32x4_NEG => Ok((bytes, Self::I32x4Neg)),
            Self::BYTE_PREFIX_I32x4_ALL_TRUE => Ok((bytes, Self::I32x4AllTrue)),
            Self::BYTE_PREFIX_I32x4_BITMASK => Ok((bytes, Self::I32x4Bitmask)),
            Self::BYTE_PREFIX_I32x4_EXTEND_LOW_I16x8_S => Ok((bytes, Self::I32x4ExtendLowI16x8S)),
            Self::BYTE_PREFIX_I32x4_EXTEND_HIGH_I16x8_S => Ok((bytes, Self::I32x4ExtendHighI16x8S)),
            Self::BYTE_PREFIX_I32x4_EXTEND_LOW_I16x8_U => Ok((bytes, Self::I32x4ExtendLowI16x8U)),
            Self::BYTE_PREFIX_I32x4_EXTEND_HIGH_I16x8_U => Ok((bytes, Self::I32x4ExtendHighI16x8U)),
            Self::BYTE_PREFIX_I32x4_SHL => Ok((bytes, Self::I32x4Shl)),
            Self::BYTE_PREFIX_I32x4_SHR_S => Ok((bytes, Self::I32x4ShrS)),
            Self::BYTE_PREFIX_I32x4_SHR_U => Ok((bytes, Self::I32x4ShrU)),
            Self::BYTE_PREFIX_I32x4_ADD => Ok((bytes, Self::I32x4Add)),
            Self::BYTE_PREFIX_I32x4_SUB => Ok((bytes, Self::I16x8Sub)),
            Self::BYTE_PREFIX_I32x4_MUL => Ok((bytes, Self::I16x8Mul)),
            Self::BYTE_PREFIX_I32x4_MIN_S => Ok((bytes, Self::I16x8MinS)),
            Self::BYTE_PREFIX_I32x4_MIN_U => Ok((bytes, Self::I16x8MinU)),
            Self::BYTE_PREFIX_I32x4_MAX_S => Ok((bytes, Self::I16x8MaxS)),
            Self::BYTE_PREFIX_I32x4_MAX_U => Ok((bytes, Self::I16x8MaxU)),
            Self::BYTE_PREFIX_I32x4_DOT_I16x8_S => Ok((bytes, Self::I16x8MaxU)),
            Self::BYTE_PREFIX_I32x4_EXTMUL_LOW_I16x8_S => Ok((bytes, Self::I32x4ExtmulLowI16x8S)),
            Self::BYTE_PREFIX_I32x4_EXTMUL_HIGH_I16x8_S => Ok((bytes, Self::I32x4ExtmulHighI16x8S)),
            Self::BYTE_PREFIX_I32x4_EXTMUL_LOW_I16x8_U => Ok((bytes, Self::I32x4ExtmulLowI16x8U)),
            Self::BYTE_PREFIX_I32x4_EXTMUL_HIGH_I16x8_U => Ok((bytes, Self::I32x4ExtmulHighI16x8U)),
            Self::BYTE_PREFIX_I64x2_ABS => Ok((bytes, Self::I64x2Abs)),
            Self::BYTE_PREFIX_I64x2_NEG => Ok((bytes, Self::I64x2Neg)),
            Self::BYTE_PREFIX_I64x2_ALL_TRUE => Ok((bytes, Self::I64x2AllTrue)),
            Self::BYTE_PREFIX_I64x2_BITMASK => Ok((bytes, Self::I64x2Bitmask)),
            Self::BYTE_PREFIX_I64x2_EXTEND_LOW_I32x4_S => Ok((bytes, Self::I64x2ExtendLowI32x4S)),
            Self::BYTE_PREFIX_I64x2_EXTEND_HIGH_I32x4_S => Ok((bytes, Self::I64x2ExtendHighI32x4S)),
            Self::BYTE_PREFIX_I64x2_EXTEND_LOW_I32x4_U => Ok((bytes, Self::I64x2ExtendLowI32x4U)),
            Self::BYTE_PREFIX_I64x2_EXTEND_HIGH_I32x4_U => Ok((bytes, Self::I64x2ExtendHighI32x4U)),
            Self::BYTE_PREFIX_I64x2_SHL => Ok((bytes, Self::I64x2Shl)),
            Self::BYTE_PREFIX_I64x2_SHR_S => Ok((bytes, Self::I64x2ShrS)),
            Self::BYTE_PREFIX_I64x2_SHR_U => Ok((bytes, Self::I64x2ShrU)),
            Self::BYTE_PREFIX_I64x2_ADD => Ok((bytes, Self::I64x2Add)),
            Self::BYTE_PREFIX_I64x2_SUB => Ok((bytes, Self::I64x2Sub)),
            Self::BYTE_PREFIX_I64x2_MUL => Ok((bytes, Self::I64x2Mul)),
            Self::BYTE_PREFIX_I64x2_EXTMUL_LOW_I32x4_S => Ok((bytes, Self::I64x2ExtmulLowI32x4S)),
            Self::BYTE_PREFIX_I64x2_EXTMUL_HIGH_I32x4_S => Ok((bytes, Self::I64x2ExtmulHighI32x4S)),
            Self::BYTE_PREFIX_I64x2_EXTMUL_LOW_I32x4_U => Ok((bytes, Self::I64x2ExtmulLowI32x4U)),
            Self::BYTE_PREFIX_I64x2_EXTMUL_HIGH_I32x4_U => Ok((bytes, Self::I64x2ExtmulHighI32x4U)),
            Self::BYTE_PREFIX_F32x4_CEIL => Ok((bytes, Self::F32x4Ceil)),
            Self::BYTE_PREFIX_F32x4_FLOOR => Ok((bytes, Self::F32x4Floor)),
            Self::BYTE_PREFIX_F32x4_TRUNC => Ok((bytes, Self::F32x4Trunc)),
            Self::BYTE_PREFIX_F32x4_NEAREST => Ok((bytes, Self::F32x4Nearest)),
            Self::BYTE_PREFIX_F32x4_ABS => Ok((bytes, Self::F32x4Abs)),
            Self::BYTE_PREFIX_F32x4_NEG => Ok((bytes, Self::F32x4Neg)),
            Self::BYTE_PREFIX_F32x4_SQRT => Ok((bytes, Self::F32x4Sqrt)),
            Self::BYTE_PREFIX_F32x4_ADD => Ok((bytes, Self::F32x4Add)),
            Self::BYTE_PREFIX_F32x4_SUB => Ok((bytes, Self::F32x4Sub)),
            Self::BYTE_PREFIX_F32x4_MUL => Ok((bytes, Self::F32x4Mul)),
            Self::BYTE_PREFIX_F32x4_DIV => Ok((bytes, Self::F32x4Div)),
            Self::BYTE_PREFIX_F32x4_MIN => Ok((bytes, Self::F32x4Min)),
            Self::BYTE_PREFIX_F32x4_MAX => Ok((bytes, Self::F32x4Max)),
            Self::BYTE_PREFIX_F32x4_PMIN => Ok((bytes, Self::F32x4Pmin)),
            Self::BYTE_PREFIX_F32x4_PMAX => Ok((bytes, Self::F32x4Pmax)),
            Self::BYTE_PREFIX_F64x2_CEIL => Ok((bytes, Self::F64x2Ceil)),
            Self::BYTE_PREFIX_F64x2_FLOOR => Ok((bytes, Self::F64x2Floor)),
            Self::BYTE_PREFIX_F64x2_TRUNC => Ok((bytes, Self::F64x2Trunc)),
            Self::BYTE_PREFIX_F64x2_NEAREST => Ok((bytes, Self::F64x2Nearest)),
            Self::BYTE_PREFIX_F64x2_ABS => Ok((bytes, Self::F64x2Abs)),
            Self::BYTE_PREFIX_F64x2_NEG => Ok((bytes, Self::F64x2Neg)),
            Self::BYTE_PREFIX_F64x2_SQRT => Ok((bytes, Self::F64x2Sqrt)),
            Self::BYTE_PREFIX_F64x2_ADD => Ok((bytes, Self::F64x2Add)),
            Self::BYTE_PREFIX_F64x2_SUB => Ok((bytes, Self::F64x2Sub)),
            Self::BYTE_PREFIX_F64x2_MUL => Ok((bytes, Self::F64x2Mul)),
            Self::BYTE_PREFIX_F64x2_DIV => Ok((bytes, Self::F64x2Div)),
            Self::BYTE_PREFIX_F64x2_MIN => Ok((bytes, Self::F64x2Min)),
            Self::BYTE_PREFIX_F64x2_MAX => Ok((bytes, Self::F64x2Max)),
            Self::BYTE_PREFIX_F64x2_PMIN => Ok((bytes, Self::F64x2Pmin)),
            Self::BYTE_PREFIX_F64x2_PMAX => Ok((bytes, Self::F64x2Pmax)),
            Self::BYTE_PREFIX_I32x4_TRUNC_SAT_F32x4_S => Ok((bytes, Self::I32x4TruncSatF32x4S)),
            Self::BYTE_PREFIX_I32x4_TRUNC_SAT_F32x4_U => Ok((bytes, Self::I32x4TruncSatF32x4U)),
            Self::BYTE_PREFIX_F32x4_CONVERT_I32x4_S => Ok((bytes, Self::F32x4ConvertI32x4S)),
            Self::BYTE_PREFIX_F32x4_CONVERT_I32x4_U => Ok((bytes, Self::F32x4ConvertI32x4U)),
            Self::BYTE_PREFIX_I32x4_TRUNC_SAT_F64x2_S_ZERO => {
                Ok((bytes, Self::I32x4TruncSatF64x2SZero))
            }
            Self::BYTE_PREFIX_I32x4_TRUNC_SAT_F64x2_U_ZERO => {
                Ok((bytes, Self::I32x4TruncSatF64x2UZero))
            }
            Self::BYTE_PREFIX_F64x2_CONVERT_LOW_I32x4_S => Ok((bytes, Self::F64x2ConvertLowI32x4S)),
            Self::BYTE_PREFIX_F64x2_CONVERT_LOW_I32x4_U => Ok((bytes, Self::F64x2ConvertLowI32x4U)),
            Self::BYTE_PREFIX_F32x4_DEMOTE_F64x2_ZERO => Ok((bytes, Self::F32x4DemoteF64x2Zero)),
            Self::BYTE_PREFIX_F64x2_PROMOTE_LOW_F32x4 => Ok((bytes, Self::F64x2PromoteLowF32x4)),

            _ => Err(nom::Err::Failure(nom::error::Error::new(
                bytes,
                nom::error::ErrorKind::Fail,
            ))),
        }
    }
}

impl ParseWithNom for InstructionType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, opcode) = take(1usize)(bytes)?;

        match opcode[0] {
            Self::OPCODE_UNREACHABLE => Ok((bytes, Self::Unreachable)),
            Self::OPCODE_NOP => Ok((bytes, Self::Nop)),
            Self::OPCODE_BLOCK => parse(bytes).map(|(b, v)| (b, Self::Block(v))),
            Self::OPCODE_LOOP => parse(bytes).map(|(b, v)| (b, Self::Loop(v))),
            Self::OPCODE_IF_ELSE => parse(bytes).map(|(b, v)| (b, Self::IfElse(v))),
            Self::OPCODE_BR => parse(bytes).map(|(b, v)| (b, Self::Br(v))),
            Self::OPCODE_BR_IF => parse(bytes).map(|(b, v)| (b, Self::BrIf(v))),
            Self::OPCODE_BR_TABLE => parse(bytes).map(|(b, v)| (b, Self::BrTable(v))),
            Self::OPCODE_RETURN => Ok((bytes, Self::Return)),
            Self::OPCODE_CALL => parse(bytes).map(|(b, v)| (b, Self::Call(v))),
            Self::OPCODE_CALL_INDIRECT => parse(bytes).map(|(b, v)| (b, Self::CallIndirect(v))),

            Self::OPCODE_REF_NULL => parse(bytes).map(|(b, v)| (b, Self::RefNull(v))),
            Self::OPCODE_REF_IS_NULL => Ok((bytes, Self::RefIsNull)),
            Self::OPCODE_REF_FUNC => parse(bytes).map(|(b, v)| (b, Self::RefFunc(v))),
            Self::OPCODE_DROP => Ok((bytes, Self::Drop)),
            Self::OPCODE_SELECT => Ok((bytes, Self::Select)),

            Self::OPCODE_SELECT_VEC => parse(bytes).map(|(b, v)| (b, Self::SelectVec(v))),
            Self::OPCODE_LOCAL_GET => parse(bytes).map(|(b, v)| (b, Self::LocalGet(v))),
            Self::OPCODE_LOCAL_SET => parse(bytes).map(|(b, v)| (b, Self::LocalSet(v))),
            Self::OPCODE_LOCAL_TEE => parse(bytes).map(|(b, v)| (b, Self::LocalTee(v))),
            Self::OPCODE_GLOBAL_GET => parse(bytes).map(|(b, v)| (b, Self::GlobalGet(v))),
            Self::OPCODE_GLOBAL_SET => parse(bytes).map(|(b, v)| (b, Self::GlobalSet(v))),

            Self::OPCODE_TABLE_GET => parse(bytes).map(|(b, v)| (b, Self::TableGet(v))),
            Self::OPCODE_TABLE_SET => parse(bytes).map(|(b, v)| (b, Self::TableSet(v))),
            Self::OPCODE_OTHER => Self::parse_other(bytes),

            Self::OPCODE_I32_LOAD => parse(bytes).map(|(b, v)| (b, Self::I32Load(v))),
            Self::OPCODE_I64_LOAD => parse(bytes).map(|(b, v)| (b, Self::I64Load(v))),
            Self::OPCODE_F32_LOAD => parse(bytes).map(|(b, v)| (b, Self::F32Load(v))),
            Self::OPCODE_F64_LOAD => parse(bytes).map(|(b, v)| (b, Self::F64Load(v))),
            Self::OPCODE_I32_LOAD_8_S => parse(bytes).map(|(b, v)| (b, Self::I32Load8S(v))),
            Self::OPCODE_I32_LOAD_8_U => parse(bytes).map(|(b, v)| (b, Self::I32Load8U(v))),
            Self::OPCODE_I32_LOAD_16_S => parse(bytes).map(|(b, v)| (b, Self::I32Load16S(v))),
            Self::OPCODE_I32_LOAD_16_U => parse(bytes).map(|(b, v)| (b, Self::I32Load16U(v))),
            Self::OPCODE_I64_LOAD_8_S => parse(bytes).map(|(b, v)| (b, Self::I64Load8S(v))),
            Self::OPCODE_I64_LOAD_8_U => parse(bytes).map(|(b, v)| (b, Self::I64Load8U(v))),
            Self::OPCODE_I64_LOAD_16_S => parse(bytes).map(|(b, v)| (b, Self::I64Load16S(v))),
            Self::OPCODE_I64_LOAD_16_U => parse(bytes).map(|(b, v)| (b, Self::I64Load16U(v))),
            Self::OPCODE_I64_LOAD_32_S => parse(bytes).map(|(b, v)| (b, Self::I64Load32S(v))),
            Self::OPCODE_I64_LOAD_32_U => parse(bytes).map(|(b, v)| (b, Self::I64Load32U(v))),
            Self::OPCODE_I32_STORE => parse(bytes).map(|(b, v)| (b, Self::I32Store(v))),
            Self::OPCODE_I64_STORE => parse(bytes).map(|(b, v)| (b, Self::I64Store(v))),
            Self::OPCODE_F32_STORE => parse(bytes).map(|(b, v)| (b, Self::F32Store(v))),
            Self::OPCODE_F64_STORE => parse(bytes).map(|(b, v)| (b, Self::F64Store(v))),
            Self::OPCODE_I32_STORE_8 => parse(bytes).map(|(b, v)| (b, Self::I32Store8(v))),
            Self::OPCODE_I32_STORE_16 => parse(bytes).map(|(b, v)| (b, Self::I32Store16(v))),
            Self::OPCODE_I64_STORE_8 => parse(bytes).map(|(b, v)| (b, Self::I64Store8(v))),
            Self::OPCODE_I64_STORE_16 => parse(bytes).map(|(b, v)| (b, Self::I64Store16(v))),
            Self::OPCODE_I64_STORE_32 => parse(bytes).map(|(b, v)| (b, Self::I64Store32(v))),
            Self::OPCODE_MEMORY_SIZE => Ok((tag(&[0x00])(bytes).map(|v| v.0)?, Self::MemorySize)),
            Self::OPCODE_MEMORY_GROW => Ok((tag(&[0x00])(bytes).map(|v| v.0)?, Self::MemoryGrow)),

            Self::OPCODE_I32_CONST => parse(bytes).map(|(b, v)| (b, Self::I32Const(v))),
            Self::OPCODE_I64_CONST => parse(bytes).map(|(b, v)| (b, Self::I64Const(v))),
            Self::OPCODE_F32_CONST => parse(bytes).map(|(b, v)| (b, Self::F32Const(v))),
            Self::OPCODE_F64_CONST => parse(bytes).map(|(b, v)| (b, Self::F64Const(v))),
            Self::OPCODE_I32_EQZ => Ok((bytes, Self::I32Eqz)),
            Self::OPCODE_I32_EQ => Ok((bytes, Self::I32Eq)),
            Self::OPCODE_I32_NE => Ok((bytes, Self::I32Ne)),
            Self::OPCODE_I32_LT_S => Ok((bytes, Self::I32LtS)),
            Self::OPCODE_I32_LT_U => Ok((bytes, Self::I32LtU)),
            Self::OPCODE_I32_GT_S => Ok((bytes, Self::I32GtS)),
            Self::OPCODE_I32_GT_U => Ok((bytes, Self::I32GtU)),
            Self::OPCODE_I32_LE_S => Ok((bytes, Self::I32LeS)),
            Self::OPCODE_I32_LE_U => Ok((bytes, Self::I32LeU)),
            Self::OPCODE_I32_GE_S => Ok((bytes, Self::I32GeS)),
            Self::OPCODE_I32_GE_U => Ok((bytes, Self::I32GeU)),
            Self::OPCODE_I64_EQZ => Ok((bytes, Self::I64Eqz)),
            Self::OPCODE_I64_EQ => Ok((bytes, Self::I64Eq)),
            Self::OPCODE_I64_NE => Ok((bytes, Self::I64Ne)),
            Self::OPCODE_I64_LT_S => Ok((bytes, Self::I64LtS)),
            Self::OPCODE_I64_LT_U => Ok((bytes, Self::I64LtU)),
            Self::OPCODE_I64_GT_S => Ok((bytes, Self::I64GtS)),
            Self::OPCODE_I64_GT_U => Ok((bytes, Self::I64GtU)),
            Self::OPCODE_I64_LE_S => Ok((bytes, Self::I64LeS)),
            Self::OPCODE_I64_LE_U => Ok((bytes, Self::I64LeU)),
            Self::OPCODE_I64_GE_S => Ok((bytes, Self::I64GeS)),
            Self::OPCODE_I64_GE_U => Ok((bytes, Self::I64GeU)),
            Self::OPCODE_F32_EQ => Ok((bytes, Self::F32Eq)),
            Self::OPCODE_F32_NE => Ok((bytes, Self::F32Ne)),
            Self::OPCODE_F32_LT => Ok((bytes, Self::F32Lt)),
            Self::OPCODE_F32_GT => Ok((bytes, Self::F32Gt)),
            Self::OPCODE_F32_LE => Ok((bytes, Self::F32Le)),
            Self::OPCODE_F32_GE => Ok((bytes, Self::F32Ge)),
            Self::OPCODE_F64_EQ => Ok((bytes, Self::F64Eq)),
            Self::OPCODE_F64_NE => Ok((bytes, Self::F64Ne)),
            Self::OPCODE_F64_LT => Ok((bytes, Self::F64Lt)),
            Self::OPCODE_F64_GT => Ok((bytes, Self::F64Gt)),
            Self::OPCODE_F64_LE => Ok((bytes, Self::F64Le)),
            Self::OPCODE_F64_GE => Ok((bytes, Self::F64Ge)),
            Self::OPCODE_I32_CLZ => Ok((bytes, Self::I32Clz)),
            Self::OPCODE_I32_CTZ => Ok((bytes, Self::I32Ctz)),
            Self::OPCODE_I32_POPCNT => Ok((bytes, Self::I32Popcnt)),
            Self::OPCODE_I32_ADD => Ok((bytes, Self::I32Add)),
            Self::OPCODE_I32_SUB => Ok((bytes, Self::I32Sub)),
            Self::OPCODE_I32_MUL => Ok((bytes, Self::I32Mul)),
            Self::OPCODE_I32_DIV_S => Ok((bytes, Self::I32DivS)),
            Self::OPCODE_I32_DIV_U => Ok((bytes, Self::I32DivU)),
            Self::OPCODE_I32_REM_S => Ok((bytes, Self::I32RemS)),
            Self::OPCODE_I32_REM_U => Ok((bytes, Self::I32RemU)),
            Self::OPCODE_I32_AND => Ok((bytes, Self::I32And)),
            Self::OPCODE_I32_OR => Ok((bytes, Self::I32Or)),
            Self::OPCODE_I32_XOR => Ok((bytes, Self::I32Xor)),
            Self::OPCODE_I32_SHL => Ok((bytes, Self::I32Shl)),
            Self::OPCODE_I32_SHR_S => Ok((bytes, Self::I32ShrS)),
            Self::OPCODE_I32_SHR_U => Ok((bytes, Self::I32ShrU)),
            Self::OPCODE_I32_ROTL => Ok((bytes, Self::I32Rotl)),
            Self::OPCODE_I32_ROTR => Ok((bytes, Self::I32Rotr)),
            Self::OPCODE_I64_CLZ => Ok((bytes, Self::I64Clz)),
            Self::OPCODE_I64_CTZ => Ok((bytes, Self::I64Ctz)),
            Self::OPCODE_I64_POPCNT => Ok((bytes, Self::I64Popcnt)),
            Self::OPCODE_I64_ADD => Ok((bytes, Self::I64Add)),
            Self::OPCODE_I64_SUB => Ok((bytes, Self::I64Sub)),
            Self::OPCODE_I64_MUL => Ok((bytes, Self::I64Mul)),
            Self::OPCODE_I64_DIV_S => Ok((bytes, Self::I64DivS)),
            Self::OPCODE_I64_DIV_U => Ok((bytes, Self::I64DivU)),
            Self::OPCODE_I64_REM_S => Ok((bytes, Self::I64RemS)),
            Self::OPCODE_I64_REM_U => Ok((bytes, Self::I64RemU)),
            Self::OPCODE_I64_AND => Ok((bytes, Self::I64And)),
            Self::OPCODE_I64_OR => Ok((bytes, Self::I64Or)),
            Self::OPCODE_I64_XOR => Ok((bytes, Self::I64Xor)),
            Self::OPCODE_I64_SHL => Ok((bytes, Self::I64Shl)),
            Self::OPCODE_I64_SHR_S => Ok((bytes, Self::I64ShrS)),
            Self::OPCODE_I64_SHR_U => Ok((bytes, Self::I64ShrU)),
            Self::OPCODE_I64_ROTL => Ok((bytes, Self::I64Rotl)),
            Self::OPCODE_I64_ROTR => Ok((bytes, Self::I64Rotr)),

            Self::OPCODE_F32_ABS => Ok((bytes, Self::F32Abs)),
            Self::OPCODE_F32_NEG => Ok((bytes, Self::F32Neg)),
            Self::OPCODE_F32_CEIL => Ok((bytes, Self::F32Ceil)),
            Self::OPCODE_F32_FLOOR => Ok((bytes, Self::F32Floor)),
            Self::OPCODE_F32_TRUNC => Ok((bytes, Self::F32Trunc)),
            Self::OPCODE_F32_NEAREST => Ok((bytes, Self::F32Nearest)),
            Self::OPCODE_F32_SQRT => Ok((bytes, Self::F32Sqrt)),
            Self::OPCODE_F32_ADD => Ok((bytes, Self::F32Add)),
            Self::OPCODE_F32_SUB => Ok((bytes, Self::F32Sub)),
            Self::OPCODE_F32_MUL => Ok((bytes, Self::F32Mul)),
            Self::OPCODE_F32_DIV => Ok((bytes, Self::F32Div)),
            Self::OPCODE_F32_MIN => Ok((bytes, Self::F32Min)),
            Self::OPCODE_F32_MAX => Ok((bytes, Self::F32Max)),
            Self::OPCODE_F32_COPYSIGN => Ok((bytes, Self::F32Copysign)),
            Self::OPCODE_F64_ABS => Ok((bytes, Self::F64Abs)),
            Self::OPCODE_F64_NEG => Ok((bytes, Self::F64Neg)),
            Self::OPCODE_F64_CEIL => Ok((bytes, Self::F64Ceil)),
            Self::OPCODE_F64_FLOOR => Ok((bytes, Self::F64Floor)),
            Self::OPCODE_F64_TRUNC => Ok((bytes, Self::F64Trunc)),
            Self::OPCODE_F64_NEAREST => Ok((bytes, Self::F64Nearest)),
            Self::OPCODE_F64_SQRT => Ok((bytes, Self::F64Sqrt)),
            Self::OPCODE_F64_ADD => Ok((bytes, Self::F64Add)),
            Self::OPCODE_F64_SUB => Ok((bytes, Self::F64Sub)),
            Self::OPCODE_F64_MUL => Ok((bytes, Self::F64Mul)),
            Self::OPCODE_F64_DIV => Ok((bytes, Self::F64Div)),
            Self::OPCODE_F64_MIN => Ok((bytes, Self::F64Min)),
            Self::OPCODE_F64_MAX => Ok((bytes, Self::F64Max)),
            Self::OPCODE_F64_COPYSIGN => Ok((bytes, Self::F64Copysign)),
            Self::OPCODE_I32_WRAP_I64 => Ok((bytes, Self::I32WrapI64)),
            Self::OPCODE_I32_TRUNC_F32_S => Ok((bytes, Self::I32TruncF32S)),
            Self::OPCODE_I32_TRUNC_F32_U => Ok((bytes, Self::I32TruncF32U)),
            Self::OPCODE_I32_TRUNC_F64_S => Ok((bytes, Self::I32TruncF64S)),
            Self::OPCODE_I32_TRUNC_F64_U => Ok((bytes, Self::I32TruncF64U)),
            Self::OPCODE_I64_EXTEND_I32_S => Ok((bytes, Self::I64ExtendI32S)),
            Self::OPCODE_I64_EXTEND_I32_U => Ok((bytes, Self::I64ExtendI32U)),
            Self::OPCODE_I64_TRUNC_F32_S => Ok((bytes, Self::I64TruncF32S)),
            Self::OPCODE_I64_TRUNC_F32_U => Ok((bytes, Self::I64TruncF32U)),
            Self::OPCODE_I64_TRUNC_F64_S => Ok((bytes, Self::I64TruncF64S)),
            Self::OPCODE_I64_TRUNC_F64_U => Ok((bytes, Self::I64TruncF64U)),
            Self::OPCODE_F32_CONVERT_I32_S => Ok((bytes, Self::F32ConvertI32S)),
            Self::OPCODE_F32_CONVERT_I32_U => Ok((bytes, Self::F32ConvertI32U)),
            Self::OPCODE_F32_CONVERT_I64_S => Ok((bytes, Self::F32ConvertI64S)),
            Self::OPCODE_F32_CONVERT_I64_U => Ok((bytes, Self::F32ConvertI64U)),
            Self::OPCODE_F32_DEMOTE_F64 => Ok((bytes, Self::F32DemoteF64)),
            Self::OPCODE_F64_CONVERT_I32_S => Ok((bytes, Self::F64ConvertI32S)),
            Self::OPCODE_F64_CONVERT_I32_U => Ok((bytes, Self::F64ConvertI32U)),
            Self::OPCODE_F64_CONVERT_I64_S => Ok((bytes, Self::F64ConvertI64S)),
            Self::OPCODE_F64_CONVERT_I64_U => Ok((bytes, Self::F64ConvertI64U)),
            Self::OPCODE_F64_PROMOTE_F32 => Ok((bytes, Self::F64PromoteF32)),
            Self::OPCODE_I32_REINTERPRET_F32 => Ok((bytes, Self::I32ReinterpretF32)),
            Self::OPCODE_I64_REINTERPRET_F64 => Ok((bytes, Self::I64ReinterpretF64)),
            Self::OPCODE_F32_REINTERPRET_I32 => Ok((bytes, Self::F32ReinterpretI32)),
            Self::OPCODE_F64_REINTERPRET_I64 => Ok((bytes, Self::F64ReinterpretI64)),
            Self::OPCODE_I32_EXTEND_8_S => Ok((bytes, Self::I32Extend8S)),
            Self::OPCODE_I32_EXTEND_16_S => Ok((bytes, Self::I32Extend16S)),
            Self::OPCODE_I64_EXTEND_8_S => Ok((bytes, Self::I64Extend8S)),
            Self::OPCODE_I64_EXTEND_16_S => Ok((bytes, Self::I64Extend16S)),
            Self::OPCODE_I64_EXTEND_32_S => Ok((bytes, Self::I64Extend32S)),

            Self::OPCODE_VECTOR_INSTRUCTIONS => Self::parse_vector_instruction(bytes),

            _ => Err(nom::Err::Failure(nom::error::Error::new(
                bytes,
                nom::error::ErrorKind::Fail,
            ))),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct BlockInstructionType {
    pub blocktype: BlockType,
    pub instructions: Vec<InstructionType>,
}

impl ParseWithNom for BlockInstructionType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, blocktype) = BlockType::parse(bytes)?;
        let (bytes, instructions) = parse_all_to_vec(bytes, InstructionType::OPCODE_END)?;

        Ok((
            bytes,
            BlockInstructionType {
                blocktype,
                instructions,
            },
        ))
    }
}

#[derive(Debug, PartialEq)]
pub struct LoopInstructionType {
    pub blocktype: BlockType,
    pub instructions: Vec<InstructionType>,
}

impl ParseWithNom for LoopInstructionType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, blocktype) = BlockType::parse(bytes)?;
        let (bytes, instructions) = parse_all_to_vec(bytes, InstructionType::OPCODE_END)?;

        Ok((
            bytes,
            LoopInstructionType {
                blocktype,
                instructions,
            },
        ))
    }
}

#[derive(Debug, PartialEq)]
pub struct IfInstructionType {
    pub blocktype: BlockType,
    pub if_instructions: Vec<InstructionType>,
}

impl ParseWithNom for IfInstructionType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, blocktype) = BlockType::parse(bytes)?;
        let (bytes, if_instructions) = parse_all_to_vec(bytes, InstructionType::OPCODE_ELSE)?;
        // let (bytes, if_instructions) = Vec::parse(bytes)?;

        Ok((
            bytes,
            IfInstructionType {
                blocktype,
                if_instructions,
            },
        ))
    }
}

#[derive(Debug, PartialEq)]
pub struct IfElseInstructionType {
    pub blocktype: BlockType,
    pub if_instructions: Vec<InstructionType>,
    pub else_instructions: Vec<InstructionType>,
}

impl ParseWithNom for IfElseInstructionType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, blocktype) = BlockType::parse(bytes)?;

        if bytes
            .iter()
            .find(|b| **b == InstructionType::OPCODE_ELSE)
            .is_some()
        {
            // if ... else ... end instruction
            let (bytes, if_instructions) = parse_all_to_vec(bytes, InstructionType::OPCODE_ELSE)?;
            let (bytes, else_instructions) = parse_all_to_vec(bytes, InstructionType::OPCODE_END)?;

            Ok((
                bytes,
                IfElseInstructionType {
                    blocktype,
                    if_instructions,
                    else_instructions,
                },
            ))
        } else {
            // if ... end (no else) instruction
            let (bytes, if_instructions) = parse_all_to_vec(bytes, InstructionType::OPCODE_END)?;

            Ok((
                bytes,
                IfElseInstructionType {
                    blocktype,
                    if_instructions,
                    else_instructions: vec![],
                },
            ))
        }
    }
}
