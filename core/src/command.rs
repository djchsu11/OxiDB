pub enum CommandType {
    Exit,
    Invalid
}

impl CommandType {
    pub fn new(val: &str) -> CommandType {
        match val.to_ascii_lowercase().as_str() {
            ".exit" => CommandType::Exit,
            _ => CommandType::Invalid
        }
    }

    pub(crate) fn do_command(&self) -> bool {
        match *self {
            CommandType::Exit => CommandType::do_exit(),
            CommandType::Invalid => CommandType::do_invalid(),
        }
    }

    fn do_exit() -> bool {
        println!("Exiting...");
        true
    }

    fn do_invalid() -> bool {
        println!("Invalid command");
        false
    }
}
