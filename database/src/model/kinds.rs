#[derive(PartialEq)]
pub enum ExecutionStatusKind{
    ExecutionFailure,
    ExecutionSuccess,
    ExecutionSuccessContinue,
    ExecutionFailureContinue,
}

#[derive(PartialEq)]
pub enum StatementKind {
    Insert,
    Delete,
    Update,
    Select,
    Create,
    Invalid
}

impl StatementKind {
    pub fn new(val: &str) -> StatementKind {
        match val.to_ascii_lowercase().as_str() {
            "insert" => StatementKind::Insert,
            "select" => StatementKind::Select,
            "delete" => StatementKind::Delete,
            "update" => StatementKind::Update,
            "create" => StatementKind::Create,
            _ => StatementKind::Invalid
        }
    }
}