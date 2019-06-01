pub enum CommandKind {
    CommandInsert,
    CommandSelect,
    CommandDelete,
    CommandUpdate,
    None
}

pub enum StatementKind {
    StatementInsert,
    StatementSelect,
    StatementDelete,
    StatementUpdate,
    None
}

#[derive(PartialEq)]
pub enum ExecutionStatusKind{
    ExecutionFailure,
    ExecutionSuccess
}

pub struct CommandSwitchKind {
    pub command: CommandKind,
    pub statement: StatementKind
}