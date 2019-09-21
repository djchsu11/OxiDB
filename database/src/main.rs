use std::io;

mod model;

fn main() {
    let mut database = model::table::Database::get_new_database();
    loop {
        let input_raw = get_user_input();
        let input = input_raw.as_str();

        if get_command_type(input.chars().next().unwrap()) {
            if input == ".exit" {
                println!("Exiting..");
                break;
            }
        }

        let status = execute_statement_or_command(input, &mut database);
        if status == model::kinds::ExecutionStatusKind::ExecutionSuccess {
            println!("Success!");
            println!("{:?}", database.registry);
        } else {
            println!("Failure.");
        }
    }
}

fn get_user_input() -> String {
    let mut input = String::new();

    println!("Type a command:");
    io::stdin()
        .read_line(&mut input)
        .expect("Please enter a command.");

    String::from(input.trim())
}

fn execute_statement_or_command(
    input: &str,
    database: &mut model::table::Database,
) -> model::kinds::ExecutionStatusKind {
    let command_char = input.chars().next().unwrap();
    if get_command_type(command_char) {
        let command = get_command_from_input(input);
        do_command(command, database)
    } else {
        let statement = get_statement_from_input(input);
        do_statement(statement, input, database)
    }
}

fn get_command_type(command_char: char) -> bool {
    if command_char == '.' {
        true
    } else {
        false
    }
}

fn get_statement_from_input(input: &str) -> model::kinds::StatementKind {
    let parsed_string = model::parse_input(input);
    model::kinds::StatementKind::new(parsed_string.first().unwrap())
}

fn get_command_from_input(input: &str) -> model::command::CommandType {
    let parsed_string = model::parse_input(input);
    model::command::CommandType::new(parsed_string.first().unwrap())
}

fn do_statement(
    statement: model::kinds::StatementKind,
    input: &str,
    database: &mut model::table::Database,
) -> model::kinds::ExecutionStatusKind {
    model::do_statement(statement, input, database)
}

fn do_command(
    command: model::command::CommandType,
    database: &mut model::table::Database,
) -> model::kinds::ExecutionStatusKind {
    model::do_command(command, database)
}
