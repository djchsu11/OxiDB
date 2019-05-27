use std::io;

enum MetaCommandKind{
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand
}

fn main() {
    loop{
        let input = get_user_input();
        let mut a = MetaCommandKind::MetaCommandUnrecognizedCommand;
        let mut firstChar = ' ';

        if !input.is_empty(){
            firstChar = input.chars().next().unwrap();
        }

        if firstChar == '.'{
            a = MetaCommandKind::MetaCommandSuccess;
        }
        match a {
            MetaCommandKind::MetaCommandSuccess =>
                println!("You made a command!"),
            MetaCommandKind::MetaCommandUnrecognizedCommand =>
                println!("That's not a command..."),
        }
    }
}

fn get_user_input() -> &str{
    let mut input = String::new();
    input.clear();
    println!("Type a command:");
    io::stdin().read_line(&mut input).expect("Please enter a command.");
    
    input.trim()

}


