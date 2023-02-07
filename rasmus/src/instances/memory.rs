use syntax::types::{Byte, MemType};

pub struct MemInst {
    pub mem_type: MemType,
    pub data: Vec<Byte>,
}
