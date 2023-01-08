use std::{cell::Cell, rc::Rc};

use crate::instructions::Instruction;

enum Value {
    Numeric(Numeric),
    Vector(Vector),
    Reference(Reference),
}

enum Numeric {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

impl Numeric {
    const DEFAULT_I32: Numeric = Numeric::I32(0);
    const DEFAULT_I64: Numeric = Numeric::I64(0);
    const DEFAULT_F32: Numeric = Numeric::F32(0.0);
    const DEFAULT_F64: Numeric = Numeric::F64(0.0);
}

struct Vector(i128);

impl Vector {
    const DEFAULT_VECTOR: Vector = Vector(0);
}

enum Reference {
    Null,
    FuncAddr(FuncAddr),
    ExternAddr(ExternAddr),
}

impl Reference {
    const DEFAULT_REFERENCE: Reference = Reference::Null;
}

struct Trap;

enum Result {
    Value(Value),
    Trap(Trap),
}

struct FuncAddr(usize);
struct TableAddr(usize);
struct MemAddr(usize);
struct GlobalAddr(usize);
struct ElemAddr(usize);
struct DataAddr(usize);
struct ExternAddr(usize);

struct Store {
    funcs: Vec<FuncInst>,
    tables: Vec<TableInst>,
    mems: Vec<MemInst>,
    globals: Vec<GlobalInst>,
    elems: Vec<ElemInst>,
    datas: Vec<DataInst>,
}

struct ModuleInst {
    func_types: Vec<FuncType>,
    func_addrs: Vec<FuncAddr>,
    table_addrs: Vec<TableAddr>,
    mem_addrs: Vec<MemAddr>,
    global_addrs: Vec<GlobalAddr>,
    elem_addrs: Vec<ElemAddr>,
    data_addrs: Vec<DataAddr>,
    exports: Vec<ExportInst>,
}

struct FuncType {
    input: Vec<ValType>,
    output: Vec<ValType>,
}

struct FuncInst {
    func_type: FuncType,
    module_ref: Rc<ModuleInst>,
    code: FuncCode,
}

struct TypeIdx(u32);
type FuncIdx = u32;
type TableIdx = u32;
type MemIdx = u32;
type GlobalIdx = u32;
type ElemIdx = u32;
type DataIdx = u32;
type LocalIdx = u32;
type LabelIdx = u32;

struct FuncCode {
    type_idx: TypeIdx,
    locals: Vec<Value>,
    body: Vec<Box<dyn Instruction>>,
}

// quite a generic declaration for host functions so far
struct HostFuncInst {
    func_type: FuncType,
    code: dyn HostFunc,
}

type ResultType = Vec<Value>;

trait HostFunc {
    fn exec(&self, vals: ResultType) -> ResultType;
}

struct Limits {
    min: u32,
    max: Option<u32>,
}

struct TableType {
    limit: Limits,
    ref_type: RefType,
}

enum RefType {
    FuncRef,
    ExternRef,
}

struct TableInst {
    table_type: TableType,
    // TODO: implement validation which checks that all references are
    // of the same type as table_type, i.e. FuncRef or ExternRef
    elements: Vec<Reference>,
}

struct MemInst {
    mem_type: Limits,
    elements: Vec<u8>,
}

impl MemInst {
    // TODO: The length of the vector always is a multiple of the WebAssembly page size, which is defined to be the constant  – abbreviated
    // https://webassembly.github.io/spec/core/exec/runtime.html#memory-instances
    const PAGE_SIZE: usize = 65_536;
}

enum MutType {
    Const,
    Var,
}

enum ValType {
    NumType(NumType),
    VecType(VecType),
    RefType(RefType),
}

enum VecType {
    V128,
}

enum NumType {
    I32,
    I64,
    F32,
    F64,
}

struct GlobalType {
    mut_type: MutType,
    val_type: ValType,
}

struct GlobalInst {
    global_type: GlobalType,
    data: Value,
}

struct ElemInst {
    elem_type: RefType,
    elem: Vec<Reference>,
}

struct DataInst {
    data: Vec<u8>,
}

struct ExportInst {
    name: String,
    value: ExternVal,
}

enum ExternVal {
    FuncAddr(FuncAddr),
    TableAddr(TableAddr),
    MemAddr(MemAddr),
    GlobalAddr(GlobalAddr),
}

struct Stack {
    stack: Vec<StackEntry>,
}

enum StackEntry {
    Value(Value),
    Label(Label),
    Activation,
}

struct Label {
    arity: u32,
    instructions: Vec<Box<dyn Instruction>>,
}

struct ActivationFrame {
    arity: u32,
    locals: Vec<Value>,
    module: Rc<Cell<ModuleInst>>,
}
