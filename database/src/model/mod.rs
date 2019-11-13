use core::mem;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;
use crate::model::query::Type;

pub mod query;
pub mod table;

pub mod command;
pub mod kinds;

pub fn do_command(
    option: command::CommandType,
    database: &mut HashMap<String, table::Table>,
) -> kinds::ExecutionStatusKind {
    kinds::ExecutionStatusKind::ExecutionSuccessContinue
}

pub fn do_statement(
    statement_type: kinds::StatementKind,
    query: &str,
    database: &mut HashMap<String, table::Table>,
) -> kinds::ExecutionStatusKind {
    let status;

    match statement_type {
        kinds::StatementKind::Create => status = handle_create(query, database),
        kinds::StatementKind::Select => status = handle_select(query, database),
        kinds::StatementKind::Insert => status = handle_insert(query, database),
        kinds::StatementKind::Update => status = handle_update(query, database),
        kinds::StatementKind::Delete => status = handle_delete(query, database),
        kinds::StatementKind::Invalid => status = kinds::ExecutionStatusKind::ExecutionFailure,
    }

    status
}

fn handle_select(
    input: &str,
    database: &mut HashMap<String, table::Table>,
) -> kinds::ExecutionStatusKind {
    let mut status = kinds::ExecutionStatusKind::ExecutionFailure;
    if check_syntax(input, query::QueryType::Select) {
        println!("Status checked");
        let query = create_query(input);
        if do_query(query, database) {
            status = kinds::ExecutionStatusKind::ExecutionSuccess
        } else {
            println!("Status checked");
            status = kinds::ExecutionStatusKind::ExecutionFailure
        }
    }
    status
}

fn handle_update(
    input: &str,
    database: &mut HashMap<String, table::Table>,
) -> kinds::ExecutionStatusKind {
    let mut status = kinds::ExecutionStatusKind::ExecutionFailure;
    if check_syntax(input, query::QueryType::Update) {
        let query = create_query(input);
        if do_query(query, database) {
            status = kinds::ExecutionStatusKind::ExecutionSuccess
        } else {
            status = kinds::ExecutionStatusKind::ExecutionFailure
        }
    }
    status
}

fn handle_insert(
    input: &str,
    database: &mut HashMap<String, table::Table>,
) -> kinds::ExecutionStatusKind {
    let mut status = kinds::ExecutionStatusKind::ExecutionFailure;
    if check_syntax(input, query::QueryType::Insert) {
        let query = parse_insert(input);
        if do_query(query, database) {
            status = kinds::ExecutionStatusKind::ExecutionSuccess
        } else {
            status = kinds::ExecutionStatusKind::ExecutionFailure
        }
    }
    status
}

fn handle_create(
    input: &str,
    database: &mut HashMap<String, table::Table>,
) -> kinds::ExecutionStatusKind {
    let mut status = kinds::ExecutionStatusKind::ExecutionFailure;
    if check_syntax(input, query::QueryType::Create) {
        let query = create_query(input);
        if do_query(query, database) {
            status = kinds::ExecutionStatusKind::ExecutionSuccess
        } else {
            status = kinds::ExecutionStatusKind::ExecutionFailure
        }
    }
    status
}

fn handle_delete(
    input: &str,
    database: &mut HashMap<String, table::Table>,
) -> kinds::ExecutionStatusKind {
    let mut status = kinds::ExecutionStatusKind::ExecutionFailure;
    if check_syntax(input, query::QueryType::Delete) {
        let query = create_query(input);
        if do_query(query, database) {
            status = kinds::ExecutionStatusKind::ExecutionSuccess
        } else {
            status = kinds::ExecutionStatusKind::ExecutionFailure
        }
    }
    status
}

//ToDo: Implement other regexes than select. Improve select regex.
fn check_syntax(input: &str, query_type: query::QueryType) -> bool {
    let select_regex = Regex::new(r"(?i)SELECT [\w\*, ]+ WHERE [\w ]+=[\w ]+;").unwrap();
    let insert_regex = Regex::new(r"(?i)INSERT INTO \w+ VALUES\s*\([\w\d\s,]+\);").unwrap();
    let delete_regex = Regex::new(r"(?i)SELECT [\w, ]+ WHERE \w = \w;").unwrap();
    let update_regex = Regex::new(r"(?i)SELECT [\w, ]+ WHERE \w = \w;").unwrap();
    let create_regex = Regex::new(r"(?i)CREATE\s+TABLE\s+\w+\s*\{\s*[\w, ]+};").unwrap();

    match query_type {
        query::QueryType::Select => select_regex.is_match(input),
        query::QueryType::Insert => insert_regex.is_match(input),
        query::QueryType::Delete => delete_regex.is_match(input),
        query::QueryType::Update => update_regex.is_match(input),
        query::QueryType::Create => create_regex.is_match(input),
        query::QueryType::Invalid => false,
    }
}

fn create_query(input: &str) -> query::Query {
    let split_input: Vec<&str> = input.split_whitespace().collect();
    let command = get_command_from_first_input(split_input[0]);

    match command {
        query::QueryType::Create => parse_create(input),
        query::QueryType::Insert => parse_insert(input),
        query::QueryType::Select => parse_select(input),
        _ => parse_create(input),
    }
}

fn get_command_from_first_input(command: &str) -> query::QueryType {
    match command.to_ascii_lowercase().as_str() {
        "select" => query::QueryType::Select,
        "create" => query::QueryType::Create,
        _ => query::QueryType::Invalid,
    }
}

fn parse_create(query: &str) -> query::Query {
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

    for column_cap in columns_group_regex.captures_iter(column_group) {
        let new_column: Vec<&str> = column_cap[0].split_whitespace().collect();
        let name = new_column[0].to_string();
        let string_column_type = new_column[1];
        let column_type;

        match string_column_type.to_ascii_lowercase().as_str() {
            "int" => column_type = query::Type::INT,
            "text" => column_type = query::Type::TEXT,
            _ => column_type = query::Type::UNKNOWN,
        }
        columns.push(query::Column {
            name,
            column_type,
            column_value: vec![0],
        });
    }

    query::Query {
        table_name: table_name.to_string(),
        columns,
        operation: query::QueryType::Create,
        all_columns_flag: false,
    }
}

fn parse_insert(query: &str) -> query::Query {
    let query_groups: Vec<&str> = query.split("(").collect();
    let table_group: Vec<&str> = query_groups[0].split_ascii_whitespace().collect();
    let value_regex = Regex::new(r"[\w\d]+,*").unwrap();

    let mut parsed_query = query::Query::new();
    //Get Table Name
    parsed_query.table_name = String::from(table_group[2]);
    //Get Query Type
    parsed_query.operation = query::QueryType::Insert;
    //Get Values
    let mut columns :Vec<query::Column> = Vec::new();
    for value_cap in value_regex.captures_iter(query_groups[1]){
        let w:&str = &value_cap[0];
        let mut  val = Vec::from(w.as_bytes());
        if val.last().unwrap() == &u8::from_str("44").unwrap(){
            val.remove(val.len() - 1);
        }
        let column = query::Column { name: String::from(""), column_type: query::Type::UNKNOWN, column_value: val };
        columns.push(column);
    }

    parsed_query.columns = columns;
    parsed_query
}

/*
* Support only * selects for now
* TODO: Add support for column selects
*/
fn parse_select(query: &str) -> query::Query {
    let query_groups: Vec<&str> = query.split("where").collect();
    let mut select_query = query::Query::new();
    let table_group = query_groups.get(0).unwrap();
    if table_group.contains("*"){
        select_query.all_columns_flag = true;
    }
    let table_name_vec: Vec<&str> = table_group.split("from").collect();
    let table_name = table_name_vec.get(1).unwrap();
    let column_cap = Regex::new(r"[\w ]+=[\w ]+").unwrap();

    select_query.table_name = table_name.trim().to_string();
    let mut columns: Vec<query::Column> = Vec::new();
    for column_cap in column_cap.captures_iter(query_groups[1]){
        let parameter:&str = &column_cap[0];
        let column_value: Vec<&str> = parameter.split("=").collect();
        println!("{:?}", column_value);
        let byte_value = String::from(column_value[1].trim()).into_bytes();
        let column = query::Column { name: String::from(column_value[0]), column_type: Type::UNKNOWN, column_value: byte_value };
        select_query.columns.push(column);
    }
    println!("{:?}", select_query);
    select_query
}

fn do_query(mut action: query::Query, mut database: &mut HashMap<String, table::Table>) -> bool {
    let mut name = "";
    let mut result = true;
    if action.operation == query::QueryType::Create {
        let mut rows: Vec<table::Cell> = Vec::new();
        for column in action.columns.iter() {
            let cell_name = &column.name;
            rows.push(table::Cell {
                name: cell_name.to_string(),
                column_type: get_table_type_from_query_type(&column.column_type),
                value: Vec::new(),
            });
        }
        let table_row = table::Row { row: rows };
        let mut table: Vec<table::Row> = Vec::new();
        table.push(table_row);
        name = &action.table_name;
        database.insert(String::from(name), table::Table { table });
    }
    if action.operation == query::QueryType::Insert {
        let mut table = database.get_mut(&action.table_name);
        match table{
            None => {
                println!("Table not found");
                result = false;
            }
            Some(x) => {
                let sample_row = &x.table[0];
                if sample_row.row.len() != action.columns.len(){
                    println!("Error: Column Mismatch");
                    result = false;
                }
                else{
                    let mut columns :Vec<table::Cell> = Vec::new();
                    let mut i = 0;
                    for column in &sample_row.row{
                        let column_type;
                        match column.column_type{
                            table::Type::TEXT => column_type = table::Type::TEXT,
                            table::Type::INT => column_type = table::Type::INT,
                            _=> column_type = table::Type::UNKNOWN,
                        };
                        let cell = table::Cell{name: String::from(&column.name), column_type, value: action.columns[i].column_value.clone()};
                        columns.push(cell);
                        i += 1;
                    }
                    x.table.push(table::Row{row: columns});
                    result = true;

                }
            }
        }
    }
    if action.operation == query::QueryType::Select {
        let mut table = database.get_mut(&action.table_name);

        match table{
            None => {
                println!("Table not found");
                result = false;
            }
            Some(x) => {
                let sample_row = &mut x.table[0];
                let mut valid = true;
                sample_row.row.sort();
                action.columns.sort();
                for column in &sample_row.row{
                    for query_column in &action.columns{
                        if query_column.name != column.name{ 
                            println!("Invalid column: {:?}", query_column.name);
                            valid = false;
                            break;
                        }
                    }
                }

                if valid{
                    // create struct to hold results
                    let mut results: Vec<&table::Row> = Vec::new();
                    // build usable set of criteria
                    let mut query_params = HashMap::new();

                    for query_column in &action.columns{
                        query_params.insert(&query_column.name, &query_column.column_value);
                    }

                    // iterate through table
                    for row in &x.table{
                        let mut valid_row = true;
                        for cell in &row.row{
                            match query_params.get(&cell.name){
                                None => valid_row = false,
                                _ => valid_row = true,
                            }
                            if !valid_row{
                                break;
                            }
                        }
                        if valid_row{
                            results.push(&row);
                        }
                    }
                    result = true;
                    //print results
                    println!("{:?}", results);

                }

            }
        }
    }
    result
}

fn get_table_type_from_query_type(query_type: &query::Type) -> table::Type {
    match query_type {
        query::Type::INT => table::Type::INT,
        query::Type::TEXT => table::Type::TEXT,
        _ => table::Type::UNKNOWN,
    }
}

pub fn parse_input(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}
