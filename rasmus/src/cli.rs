pub struct UserInput {
    pub source_file_path: String,
}

impl UserInput {
    pub fn parse_args() -> UserInput {
        UserInput {
            source_file_path: "./rasmus/tests/files/factorial.wasm".into(),
        }
    }
}
