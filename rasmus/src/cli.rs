use std::env::args;
use std::path::Path;

pub struct UserInput {
    pub source_file_path: String,
}

impl UserInput {
    pub fn parse_args() -> ::std::io::Result<Self> {
        for arg in args().skip(1) {
            println!("{arg}")
        }

        Ok(UserInput {
            source_file_path: "./wasm_files/complete_module.wasm".into(),
        })
    }
}
