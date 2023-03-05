pub mod lib;

use crate::lib::generate_command_list;
use crate::lib::execute_command_list;

fn main() {
    use std::env;

    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let command_list = generate_command_list(file_path);

    execute_command_list(command_list);
}


