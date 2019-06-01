pub enum StatementType {
    Insert,
    Delete,
    Update,
    Select,
}

impl StatementType {
    pub(crate) fn do_statement(&self) -> bool {
        match *self {
            StatementType::Insert => StatementType::do_insert(),
            StatementType::Select => StatementType::do_select(),
            StatementType::Update => StatementType::do_update(),
            StatementType::Delete => StatementType::do_delete(),
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
