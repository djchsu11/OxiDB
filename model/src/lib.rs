use regex::Regex;
use std::collections::HashMap;
mod table;
mod query;

static mut database: Option<HashMap<String, table::Table>> = None;

pub fn handle_select(input: &str) -> bool {
    check_syntax(input, query::StatementType::Select)

}

pub fn handle_insert(input: &str) -> bool {
    println!("{}", input);
    true
}

pub fn handle_create(input: &str) -> bool {
    check_syntax(input, query::StatementType::Create);
    match &database{
        Some(x) => println!("Database already initialized"),
        None => database = Some(HashMap::new()),
    }
    let query = create_query(input);

    return true
}

//ToDo: Implement other regexes than select. Improve select regex.
fn check_syntax(input: &str, query_type: query::StatementType) -> bool{
    let select_regex = Regex::new(r"(?i)SELECT [\w, ]+ WHERE \w = \w;").unwrap();
    let insert_regex = Regex::new(r"(?i)SELECT [\w, ]+ WHERE \w = \w;").unwrap();
    let delete_regex = Regex::new(r"(?i)SELECT [\w, ]+ WHERE \w = \w;").unwrap();
    let update_regex = Regex::new(r"(?i)SELECT [\w, ]+ WHERE \w = \w;").unwrap();
    let create_regex = Regex::new(r"(?i)CREATE\s+TABLE\s+\w+\s*{\s*[\w, ]+};").unwrap();

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
    let split_input: Vec<&str> = input.split_whitespace().collect();
    let command = get_command_from_first_input(split_input[0]);
    
    match command{
        query::StatementType::Create => parse_create(input),
        _=> parse_create(input),
    }
    
}

fn get_command_from_first_input(command: &str) -> query::StatementType{
    match command.to_ascii_lowercase().as_str(){
        "select" => query::StatementType::Select,
        "create" => query::StatementType::Create,
        _=> query::StatementType::Invalid

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

    query::Query{table_name:table_name.to_string(), columns, operation:query::StatementType::Create, all_columns_flag:false}
}

fn do_query(action: query::Query) -> bool{
    let mut result = None;
    let mut name = "";
    if action.operation == query::StatementType::Create{
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
        Some(x) => {
            &database.unwrap().insert(name.to_string(), x);
            true},
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

