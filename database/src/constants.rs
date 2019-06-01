//Command List
pub enum Command {
    Exit,
    Insert,
    Invalid,
}

impl Command {
    pub fn new(val: &str) -> Command {
        match val {
            ".exit" => Command::Exit,
            ".insert" => Command::Insert,
            _ => Command::Invalid
        }
    }
}
