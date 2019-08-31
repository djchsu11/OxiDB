extern crate core;
pub mod command;
pub mod statement;
pub mod kinds;

pub fn do_command(option: command::CommandType) -> kinds::ExecutionStatusKind{
    if option.do_command(){
        kinds::ExecutionStatusKind::ExecutionSuccess
    }
    else{
        kinds::ExecutionStatusKind::ExecutionFailure
    }
}

pub fn do_statement(kind: statement::StatementType, query: &str) -> kinds::ExecutionStatusKind{
    if kind.do_statement(query){
        kinds::ExecutionStatusKind::ExecutionSuccess
    }
    else{
        kinds::ExecutionStatusKind::ExecutionFailure
    }
}

pub fn parse_input(input: &str)-> Vec<&str>{
    input.split_whitespace().collect()
}

