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
        let status = do_command(command_type, input);

        if status == kinds::ExecutionStatusKind::ExitSuccess {
            println!("Exiting...");
            break;
        }
    }
}

fn execute_command(command: String) -> kinds::ExecutionStatusKind {
    if command.to_ascii_lowercase() == constants::EXIT_COMMAND.to_string() {
        kinds::ExecutionStatusKind::ExitSuccess
    } else if command.to_ascii_lowercase() == constants::MAKE_INSERT.to_string() {
        core::test();
        kinds::ExecutionStatusKind::ExitSuccess

    }else{
        println!("Unrecognized command: {}", command);
        kinds::ExecutionStatusKind::ExitFailure
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

fn do_command(command_type: kinds::MetaCommandKind, input: String) -> kinds::ExecutionStatusKind {
    match command_type {
        kinds::MetaCommandKind::MetaCommandSuccess =>
            execute_command(input),
        kinds::MetaCommandKind::MetaCommandUnrecognizedCommand =>
            {
                println!("That's not a command...");
                kinds::ExecutionStatusKind::ExitFailure
            },
    }
}
