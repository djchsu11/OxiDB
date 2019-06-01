extern crate core;
mod statement;

pub enum StatementKind{
    StatementInsert,
    StatementSelect,
    StatementDelete,
    StatementUpdate,
}

#[derive(PartialEq)]
pub enum ExecutionStatusKind{
    ExitFailure,
    ExitSuccess
}

pub fn prepare_statement(option:StatementKind) -> ExecutionStatusKind{
    let mut result = false;
    match option{
        StatementKind::StatementInsert =>
            result = statement::QueryType::Insert.do_query(),
        StatementKind::StatementSelect =>
            result = statement::QueryType::Select.do_query(),
        StatementKind::StatementUpdate =>
            result = statement::QueryType::Update.do_query(),
        StatementKind::StatementDelete =>
            result = statement::QueryType::Delete.do_query(),
    }
    if result{
        ExecutionStatusKind::ExitSuccess
    }
    else{
        ExecutionStatusKind::ExitFailure
    }
}

