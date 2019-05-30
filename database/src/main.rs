use std::io;
use core;
mod constants;

enum MetaCommandKind{
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand
}

#[derive(PartialEq)]
enum ExecutionStatusKind{
    ExitFailure,
    ExitSuccess
}

fn main() {
    loop{
        let input = get_user_input();
        let mut a = MetaCommandKind::MetaCommandUnrecognizedCommand;
        let mut first_char = ' ';
        let mut status = ExecutionStatusKind::ExitFailure;

        if !input.is_empty(){
            first_char = input.chars().next().unwrap();
        }

        if first_char == '.'{
            a = MetaCommandKind::MetaCommandSuccess;
        }
        match a {
            MetaCommandKind::MetaCommandSuccess =>
                status = execute_command(input),
            MetaCommandKind::MetaCommandUnrecognizedCommand =>
                println!("That's not a command..."),
        }

        if status == ExecutionStatusKind::ExitSuccess{
            println!("Exiting...");
            break;
        }
    }
}

fn execute_command(command: String) -> ExecutionStatusKind{
    if command.to_ascii_lowercase() == constants::EXIT_COMMAND{
        ExecutionStatusKind::ExitSuccess
    }else{
        println!("Unrecognized command: {}", command);
        ExecutionStatusKind::ExitFailure
    }
}

fn get_user_input() -> String{
    let mut input = String::new();
    input.clear();
    println!("Type a command:");
    io::stdin().read_line(&mut input).expect("Please enter a command.");
    
    String::from(input.trim())

}


