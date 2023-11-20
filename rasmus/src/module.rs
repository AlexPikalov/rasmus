use crate::binary::{module_parser::ModuleParser, parse_trait::ParseBin};
use std::fs::read;
use std::path::Path;

pub struct ModuleInstance;

impl ModuleInstance {
    pub fn from_file<P: AsRef<Path>>(path: P) -> ::std::io::Result<Self> {
        println!("Parsing");
        let file_content = read(path)?;
        let (_, module) = ModuleParser::parse(&file_content).unwrap();
        println!("module {:#?}", module);

        Ok(ModuleInstance {})
    }
}
