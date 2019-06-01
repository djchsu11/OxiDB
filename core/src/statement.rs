pub(crate) enum QueryType {
    Insert,
    Delete,
    Update,
    Select,
}

impl QueryType {
    pub(crate) fn do_query(&self) -> bool {
        match *self {
            QueryType::Insert => QueryType::do_insert(),
            QueryType::Select => QueryType::do_select(),
            QueryType::Update => QueryType::do_update(),
            QueryType::Delete => QueryType::do_delete(),
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
