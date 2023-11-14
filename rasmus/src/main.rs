mod address;
mod cli;
mod execute;
mod instances;
mod module;
mod module_registry;
mod result;
pub mod sign;

#[cfg(test)]
mod execute_test;
#[cfg(test)]
mod test_utils;

fn main() {
    println!("Hello, world!");
    let user_input = cli::UserInput::parse_args();
    module::ModuleInstance::from_file(user_input.source_file_path).unwrap();
}
