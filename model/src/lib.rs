use regex::Regex;
mod table;
mod query;

pub fn handle_select(input: &str) -> bool {
    check_syntax(input, query::StatementType::Select)

}

pub fn handle_insert(input: &str) -> bool {
    println!("{}", input);
    true
}

pub fn handle_create(input: &str) -> bool {
    check_syntax(input, query::StatementType::Create)
}

//ToDo: Implement other regexes than select. Improve select regex.
fn check_syntax(input: &str, query_type: query::StatementType) -> bool{
    let select_regex = Regex::new(r"SELECT [\w, ]+ WHERE \w = \w;").unwrap();
    let insert_regex = Regex::new(r"SELECT [\w, ]+ WHERE \w = \w;").unwrap();
    let delete_regex = Regex::new(r"SELECT [\w, ]+ WHERE \w = \w;").unwrap();
    let update_regex = Regex::new(r"SELECT [\w, ]+ WHERE \w = \w;").unwrap();
    let create_regex = Regex::new(r"CREATE\s+TABLE\s+\w+\s*{\s*[\w, ]+};").unwrap();

    match query_type{
        query::StatementType::Select => select_regex.is_match(input),
        query::StatementType::Insert => insert_regex.is_match(input),
        query::StatementType::Delete => delete_regex.is_match(input),
        query::StatementType::Update => update_regex.is_match(input),
        query::StatementType::Create => create_regex.is_match(input),
        query::StatementType::Invalid => false

    }
}    

fn create_query(input: &str) -> query::Query{
    let split_input = input.split_whitespace().collect();
    let command = get_command_from_first_input(split_input[0]);

    if (command == query::StatementType::Select){
        let all_columns_flag = true;
    }
    else{
        let all_columns_flag = false;
    }
    
}

fn get_command_from_first_input(command: &str) -> query::StatementType{
    match command.to_ascii_lowercase(){
        "select" => query::StatementType::Select,
        "create" => query::StatementType::Create,
        _=> query::StatementType::Invalid

    }
}

