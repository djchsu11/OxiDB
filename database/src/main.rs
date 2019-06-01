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

        if status == core::kinds::ExecutionStatusKind::ExitSuccess {
            println!("Exiting...");
            break;
        }
    }
}

fn execute_command(command: constants::Command) -> core::kinds::ExecutionStatusKind {
    match command {
        constants::Command::Exit =>
            core::kinds::ExecutionStatusKind::ExitSuccess,
        constants::Command::Insert =>
            core::do_command(core::kinds::CommandKind::CommandInsert),
        _ => {
            println!("Unrecognized command.");
            core::kinds::ExecutionStatusKind::ExitFailure
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

fn do_command(command_type: kinds::MetaCommandKind, input: &str) -> core::kinds::ExecutionStatusKind {
    match command_type {
        kinds::MetaCommandKind::MetaCommandSuccess =>
            {
                execute_command(constants::Command::new(input))
            },
        kinds::MetaCommandKind::MetaCommandUnrecognizedCommand =>
            {
                println!("That's not a command...");
                core::kinds::ExecutionStatusKind::ExitFailure
            },
    }
}

fn get_command_from_input(input: &str) -> core::kinds::CommandSwitchKind {
    let parsed_string = core::parse_input(input);
    let mut c;
    if parsed_string.get(0).unwrap().to_ascii_lowercase().as_str() == "select" {
        c = core::kinds::CommandSwitchKind {
            command: core::kinds::CommandKind::None,
            statement: core::kinds::StatementKind::StatementSelect,
        };
    } else {
        c = core::kinds::CommandSwitchKind {
            command: core::kinds::CommandKind::None,
            statement: core::kinds::StatementKind::None,
        };
    }
    c
}
