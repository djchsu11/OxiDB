use regex::Regex;
pub mod table;
pub mod query;

pub mod command;
pub mod kinds;
pub mod statement;

pub fn handle_select(input: &str) -> bool {
    check_syntax(input, query::QueryType::Select)

}

pub fn handle_insert(input: &str) -> bool {
    println!("{}", input);
    true
}

pub fn handle_create(input: &str) -> bool {
    check_syntax(input, query::QueryType::Create);
    let query = create_query(input);

    return true
}

//ToDo: Implement other regexes than select. Improve select regex.
fn check_syntax(input: &str, query_type: query::QueryType) -> bool{
    let select_regex = Regex::new(r"(?i)SELECT [\w, ]+ WHERE \w = \w;").unwrap();
    let insert_regex = Regex::new(r"(?i)SELECT [\w, ]+ WHERE \w = \w;").unwrap();
    let delete_regex = Regex::new(r"(?i)SELECT [\w, ]+ WHERE \w = \w;").unwrap();
    let update_regex = Regex::new(r"(?i)SELECT [\w, ]+ WHERE \w = \w;").unwrap();
    let create_regex = Regex::new(r"(?i)CREATE\s+TABLE\s+\w+\s*{\s*[\w, ]+};").unwrap();

    match query_type{
        query::QueryType::Select => select_regex.is_match(input),
        query::QueryType::Insert => insert_regex.is_match(input),
        query::QueryType::Delete => delete_regex.is_match(input),
        query::QueryType::Update => update_regex.is_match(input),
        query::QueryType::Create => create_regex.is_match(input),
        query::QueryType::Invalid => false

    }
}    

fn create_query(input: &str) -> query::Query{
    let split_input: Vec<&str> = input.split_whitespace().collect();
    let command = get_command_from_first_input(split_input[0]);
    
    match command{
        query::QueryType::Create => parse_create(input),
        _=> parse_create(input),
    }
    
}

fn get_command_from_first_input(command: &str) -> query::QueryType{
    match command.to_ascii_lowercase().as_str(){
        "select" => query::QueryType::Select,
        "create" => query::QueryType::Create,
        _=> query::QueryType::Invalid

    }
}

fn parse_create(query: &str) -> query::Query{
    let table_name_regex = Regex::new(r"(?i)CREATE TABLE \w+").unwrap();
    let columns_group_regex = Regex::new(r"\w+\s*\w+").unwrap();
    let mut columns: Vec<query::Column> = Vec::new();

    let split_query: Vec<&str> = query.split("{").collect();
    let column_group = split_query[1];


    let cap = table_name_regex.captures(query).unwrap();
    let mut table_name_iter = cap[0].split_whitespace();
    table_name_iter.next();
    table_name_iter.next();
    let table_name = table_name_iter.next().unwrap();

    println!("Table Name: {}", table_name);
    println!("Column Group: {}", column_group);

    for column_cap in columns_group_regex.captures_iter(column_group){
        let new_column: Vec<&str> = column_cap[0].split_whitespace().collect();
        let name = new_column[0].to_string();
        let string_column_type = new_column[1];
        let column_type;

        match string_column_type.to_ascii_lowercase().as_str() {
            "int" => column_type = query::Type::INT,
            "text" => column_type = query::Type::TEXT,
            _=> column_type = query::Type::UNKNOWN,
        }
        columns.push(query::Column{name, column_type, column_value:vec![0]});
    }

    query::Query{table_name:table_name.to_string(), columns, operation:query::QueryType::Create, all_columns_flag:false}
}

fn do_query(action: query::Query) -> bool{
    let mut result = None;
    let mut name = "";
    if action.operation == query::QueryType::Create{
        let mut rows: Vec<table::Cell> = Vec::new();
        for column in action.columns.iter(){
            let cell_name = &column.name;
            rows.push(table::Cell{name:cell_name.to_string(), column_type: get_table_type_from_query_type(&column.column_type), value: Vec::new()});
        }
        let table_row = table::Row{row: rows};
        let mut table : Vec<table::Row> = Vec::new();
        table.push(table_row);
        name = &action.table_name;
        result = table::Table::create_table(name.to_string(), table);
    }

    match result {
        Some(x) => true,
        None => false
    }
}

fn get_table_type_from_query_type(query_type: &query::Type) -> table::Type{
    match query_type{
        query::Type::INT => table::Type::INT,
        query::Type::TEXT => table::Type::TEXT,
        _=> table::Type::UNKNOWN
    }
}

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

