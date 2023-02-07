use super::data::DataInst;
use super::elem::ElemInst;
use super::func::FuncInst;
use super::global::GlobalInst;
use super::memory::MemInst;
use super::table::TableInst;

pub struct Store {
    pub fucns: Vec<FuncInst>,
    pub tables: Vec<TableInst>,
    pub mems: Vec<MemInst>,
    pub globals: Vec<GlobalInst>,
    pub elems: Vec<ElemInst>,
    pub datas: Vec<DataInst>,
}
