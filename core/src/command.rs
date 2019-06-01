pub(crate) enum CommandType {
    Insert,
    Delete,
    Update,
    Select,
}

impl CommandType {
    pub(crate) fn do_command(&self) -> bool {
        match *self {
            CommandType::Insert => CommandType::do_insert(),
            CommandType::Select => CommandType::do_select(),
            CommandType::Update => CommandType::do_update(),
            CommandType::Delete => CommandType::do_delete(),
        }
    }

    fn do_insert() -> bool {
        println!("Doing insert");
        true
    }

    fn do_select() -> bool {
        println!("Doing select");
        true
    }

    fn do_update() -> bool {
        println!("Doing update");
        true
    }

    fn do_delete() -> bool {
        println!("Doing delete");
        true
    }
}
