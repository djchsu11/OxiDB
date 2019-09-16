use std::io;

mod model;

fn main() {
    loop {
        if execute_statement_or_command(get_user_input().as_str())
            == model::kinds::ExecutionStatusKind::ExecutionSuccess {
            println!("Exiting...");
            break;
        }
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

fn execute_statement_or_command(input: &str) -> model::kinds::ExecutionStatusKind {
    let command_char = input.chars().next().unwrap();
    if get_command_type(command_char) {
        let command = get_command_from_input(input);
        do_command(command)
    } else {
        let statement = get_statement_from_input(input);
        do_statement(statement, input)
    }
}

fn do_command(command: model::command::CommandType) -> model::kinds::ExecutionStatusKind {
    model::do_command(command)
}

fn do_statement(statement: model::statement::StatementType, input: &str) -> model::kinds::ExecutionStatusKind {
    model::do_statement(statement, input)
}

fn get_command_from_input(input: &str) -> model::command::CommandType {
    let parsed_string = model::parse_input(input);
    model::command::CommandType::new(parsed_string.first().unwrap())
}

fn get_statement_from_input(input: &str) -> model::statement::StatementType {
    let parsed_string = model::parse_input(input);
    model::statement::StatementType::new(parsed_string.first().unwrap())
}
