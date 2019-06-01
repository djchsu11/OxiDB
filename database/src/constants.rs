//Command List
#[derive(PartialEq)]
pub enum Command {
    Exit,
    Insert,
    Select,
    Delete,
    Update,
    Invalid,
}

impl Command {
    pub fn new(val: &str) -> Command {
        match val.to_ascii_lowercase().as_str() {
            ".exit" => Command::Exit,
            "insert" => Command::Insert,
            "select" => Command::Select,
            "delete" => Command::Delete,
            "update" => Command::Update,
            _ => Command::Invalid
        }
    }
}

//Statement List
#[derive(PartialEq)]
pub enum Statement {
    Insert,
    Select,
    Delete,
    Update,
    Invalid,
}

impl Statement {
    pub fn new(val: &str) -> Statement {
        match val.to_ascii_lowercase().as_str() {
            "insert" => Statement::Insert,
            "select" => Statement::Select,
            "delete" => Statement::Delete,
            "update" => Statement::Update,
            _ => Statement::Invalid
        }
    }
}
