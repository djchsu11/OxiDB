extern crate core;
mod command;
pub mod kinds;

pub fn do_command(option: kinds::CommandKind) -> kinds::ExecutionStatusKind{
    let mut result = false;
    match option{
        kinds::CommandKind::CommandInsert =>
            result = command::CommandType::Insert.do_command(),
        kinds::CommandKind::CommandSelect =>
            result = command::CommandType::Select.do_command(),
        kinds::CommandKind::CommandUpdate =>
            result = command::CommandType::Update.do_command(),
        kinds::CommandKind::CommandDelete =>
            result = command::CommandType::Delete.do_command(),
        _=> result = false
    }
    if result{
        kinds::ExecutionStatusKind::ExecutionSuccess
    }
    else{
        kinds::ExecutionStatusKind::ExecutionFailure
    }
}

pub fn parse_input(input: &str)-> Vec<&str>{
    input.split_whitespace().collect()
}

