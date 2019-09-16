
pub enum StatementType {
    Insert,
    Delete,
    Update,
    Select,
    Create,
    Invalid
}

impl StatementType {
    pub fn new(val: &str) -> StatementType {
        match val.to_ascii_lowercase().as_str() {
            "insert" => StatementType::Insert,
            "select" => StatementType::Select,
            "delete" => StatementType::Delete,
            "update" => StatementType::Update,
            "create" => StatementType::Create,
            _ => StatementType::Invalid
        }
    }

    pub(crate) fn do_statement(&self, query: &str) -> bool {
        match *self {
            StatementType::Insert => StatementType::do_insert(query),
            StatementType::Select => StatementType::do_select(query),
            StatementType::Update => StatementType::do_update(query),
            StatementType::Delete => StatementType::do_delete(query),
            StatementType::Create => StatementType::do_create(query),
            StatementType::Invalid => StatementType::do_reject(),
        }
    }

    fn do_insert(query: &str) -> bool {
        println!("Executing query: {}", query);
        true
    }

    fn do_select(query: &str) -> bool {
        println!("Executing query: {}", query);
        true
    }

    fn do_update(query: &str) -> bool {
        println!("Executing query: {}", query);
        true
    }

    fn do_delete(query: &str) -> bool {
        println!("Executing query: {}", query);
        true
    }

    fn do_create(query: &str) -> bool {
        println!("Executing query: {}", query);
        true
    }

    fn do_reject() -> bool {
        println!("Invalid SQL");
        false
    }

    
}
