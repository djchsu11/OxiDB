use std::io;

use core;

mod constants;
mod kinds;


fn main() {
    loop{
        let input = get_user_input();

        if input.is_empty() {
            println!("Please enter a command.");
            continue;
        }

        let first_char = input.chars().next().unwrap();
        let command_type = get_command_type(first_char);
        let status = do_command(command_type, input.as_str());

        if status == core::ExecutionStatusKind::ExitSuccess {
            println!("Exiting...");
            break;
        }
    }
}

fn execute_command(command: constants::Command) -> core::ExecutionStatusKind {
    match command {
        constants::Command::Exit =>
            core::ExecutionStatusKind::ExitSuccess,
        constants::Command::Insert =>
            core::prepare_statement(core::StatementKind::StatementInsert),
        _ => {
            println!("Unrecognized command.");
            core::ExecutionStatusKind::ExitFailure
        },
    }
}

fn get_user_input() -> String{
    let mut input = String::new();

    println!("Type a command:");
    io::stdin().read_line(&mut input).expect("Please enter a command.");
    
    String::from(input.trim())

}

fn get_command_type(command_char: char) -> kinds::MetaCommandKind {
    if command_char == '.' {
        kinds::MetaCommandKind::MetaCommandSuccess
    } else {
        kinds::MetaCommandKind::MetaCommandUnrecognizedCommand
    }
}

fn do_command(command_type: kinds::MetaCommandKind, input: &str) -> core::ExecutionStatusKind {
    match command_type {
        kinds::MetaCommandKind::MetaCommandSuccess =>
            {
                execute_command(constants::Command::new(input))
            },
        kinds::MetaCommandKind::MetaCommandUnrecognizedCommand =>
            {
                println!("That's not a command...");
                core::ExecutionStatusKind::ExitFailure
            },
    }
}
