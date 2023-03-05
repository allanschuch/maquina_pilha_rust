
use std::{fs};

#[derive(Debug)]
pub enum Command {
    Push(f32),
    Sum,
    Sub,
    Div,
    Mult,
    Print
}

pub fn generate_command_list(file_path: &String) -> Vec<Command>{
    
    let file_contents = fs::read_to_string(file_path) //read the input file to a string
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = file_contents.split("\n").collect(); //split the input in a vector of strings representing its lines
    let mut command_list: Vec<Command> = Vec::new();

    //for each line, split it using whitespace as reference to obtain a "command[]" vector, where
    //command[0] = String representing the command type ("PUSH", "ADD", "DIV", etc)
    //command[1] = if the command type is push, here is the number argument
    //creates a Command object and adds to the command_list vector

    for line in lines{
        let command: Vec<&str>= line.split_whitespace().collect();
        let command_type;
        match command[0] {
            "PUSH" => {
                command_type = Command::Push(command[1].trim().parse::<f32>().unwrap());
                command_list.push(command_type)
            },
            "SUM" => {
                command_type = Command::Sum;
                command_list.push(command_type)
            },
            "SUB" => {
                command_type = Command::Sub;
                command_list.push(command_type)
            },
            "DIV" => {
                command_type = Command::Div;
                command_list.push(command_type)
            },
            "MULT" => {
                command_type = Command::Mult;
                command_list.push(command_type)
            },
            "PRINT" => {
                command_type = Command::Print;
                command_list.push(command_type)
            },
            _ => panic!("Invalid command on input file"),
        }
    }

    return command_list;
}

pub fn execute_command_list(command_list: Vec<Command>){
    let mut stack: Vec<f32> = Vec::new();

    for command in command_list{
        match command {
            Command::Push(number) => stack.push(number),
            Command::Sum => {
                let number_1 = stack.pop().unwrap();
                let number_2 = stack.pop().unwrap();
                stack.push(number_1 + number_2);
            },
            Command::Sub => {
                let number_1 = stack.pop().unwrap();
                let number_2 = stack.pop().unwrap();
                stack.push(number_1 - number_2);
            },
            Command::Div => {
                let number_1 = stack.pop().unwrap();
                let number_2 = stack.pop().unwrap();
                stack.push(number_1 / number_2);
            },
            Command::Mult => {
                let number_1 = stack.pop().unwrap();
                let number_2 = stack.pop().unwrap();
                stack.push(number_1 * number_2);
            },
            Command::Print => {
                println!("Result: {}", stack.pop().unwrap());
            },
        }
    }
}

