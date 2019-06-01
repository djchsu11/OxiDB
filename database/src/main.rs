use std::io;

use core;

mod constants;


fn main() {
    loop {
        if execute_statement_or_command(get_user_input().as_str())
            == core::kinds::ExecutionStatusKind::ExecutionSuccess {
            println!("Exiting...");
            break;
        }
    }
}

fn execute_command(command: constants::Command) -> core::kinds::ExecutionStatusKind {
    match command {
        constants::Command::Exit =>
            core::do_command(core::command::CommandType::Exit),
        constants::Command::Invalid =>
            core::do_command(core::command::CommandType::Invalid),
    }
}

fn get_user_input() -> String {
    let mut input = String::new();

    println!("Type a command:");
    io::stdin().read_line(&mut input).expect("Please enter a command.");

    String::from(input.trim())
}

fn get_command_type(command_char: char) -> bool {
    if command_char == '.' {
        true
    } else {
        false
    }
}

fn execute_statement_or_command(input: &str) -> core::kinds::ExecutionStatusKind {
    let command_char = input.chars().next().unwrap();
    if get_command_type(command_char) {
        let command = get_command_from_input(input);
        do_command(command)
    } else {
        let statement = get_statement_from_input(input);
        do_statement(statement)
    }
}

fn do_command(command: constants::Command) -> core::kinds::ExecutionStatusKind {
    if command == constants::Command::Invalid {
        println!("Unrecognized Command");
        core::kinds::ExecutionStatusKind::ExecutionFailure
    } else {
        execute_command(command)
    }
}

fn do_statement(statement: constants::Statement) -> core::kinds::ExecutionStatusKind {
    core::kinds::ExecutionStatusKind::ExecutionSuccess
}

fn get_command_from_input(input: &str) -> constants::Command {
    let parsed_string = core::parse_input(input);
    constants::Command::new(parsed_string.first().unwrap())
}

fn get_statement_from_input(input: &str) -> constants::Statement {
    let parsed_string = core::parse_input(input);
    constants::Statement::new(parsed_string.first().unwrap())
}
