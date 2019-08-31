use model;

pub enum StatementType {
    Insert,
    Delete,
    Update,
    Select,
    Invalid
}

impl StatementType {
    pub fn new(val: &str) -> StatementType {
        match val.to_ascii_lowercase().as_str() {
            "insert" => StatementType::Insert,
            "select" => StatementType::Select,
            "delete" => StatementType::Delete,
            "update" => StatementType::Update,
            _ => StatementType::Invalid
        }
    }

    pub(crate) fn do_statement(&self, query: &str) -> bool {
        match *self {
            StatementType::Insert => StatementType::do_insert(query),
            StatementType::Select => StatementType::do_select(query),
            StatementType::Update => StatementType::do_update(query),
            StatementType::Delete => StatementType::do_delete(query),
            StatementType::Invalid => StatementType::do_reject(),
        }
    }

    fn do_insert(query: &str) -> bool {
        model::handle_insert(query)
    }

    fn do_select(query: &str) -> bool {
        model::handle_select(query)
    }

    fn do_update(query: &str) -> bool {
        println!("Executing query: {}", query);
        true
    }

    fn do_delete(query: &str) -> bool {
        println!("Executing query: {}", query);
        true
    }

    fn do_reject() -> bool {
        println!("Invalid SQL");
        false
    }
}
