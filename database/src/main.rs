use std::io;

fn main() {
    let mut input = String::new();

    println!("Type something >:");
    io::stdin().read_line(&mut input).unwrap();

    println!("You typed: {}", input.trim());
}
