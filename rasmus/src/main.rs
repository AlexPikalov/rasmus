mod address;
mod cli;
mod instances;
mod module;
mod result;

fn main() {
    println!("Hello, world!");
    let user_input = cli::UserInput::parse_args();
    module::ModuleInstance::from_file(user_input.source_file_path).unwrap();
}
