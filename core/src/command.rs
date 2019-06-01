pub enum CommandType {
    Exit,
    Invalid
}

impl CommandType {
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
