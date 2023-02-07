use super::ref_inst::RefInst;
use syntax::types::TableType;

pub struct TableInst {
    pub table_type: TableType,
    pub elem: Vec<RefInst>,
}
