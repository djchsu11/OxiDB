use std::io;

fn main() {
    let mut input = String::new();

    loop{
        input.clear();
        println!("Type a command:");
        io::stdin().read_line(&mut input).unwrap();

        let character = input.trim().chars().next().unwrap();       
        if character == '.'{
            println!("You made a command!");
            break;
        }
        else{
            println!("Unrecognized Command.");
        }
        
    }
}
