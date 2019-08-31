use regex::Regex;
mod types;


pub fn handle_select(input: &str) -> bool {
    check_syntax(input, types::StatementType::Select)
}

pub fn handle_insert(input: &str) -> bool {
    println!("{}", input);
    true
}

//ToDo: Implement other regexes than select. Improve select regex.
fn check_syntax(input: &str, query_type: types::StatementType) -> bool{
    let select_regex = Regex::new(r"SELECT [\w, ]+ WHERE \w = \w;").unwrap();
    let insert_regex = Regex::new(r"SELECT [\w, ]+ WHERE \w = \w;").unwrap();
    let delete_regex = Regex::new(r"SELECT [\w, ]+ WHERE \w = \w;").unwrap();
    let update_regex = Regex::new(r"SELECT [\w, ]+ WHERE \w = \w;").unwrap();

    match query_type{
        types::StatementType::Select => select_regex.is_match(input),
        types::StatementType::Insert => insert_regex.is_match(input),
        types::StatementType::Delete => delete_regex.is_match(input),
        types::StatementType::Update => update_regex.is_match(input),
        types::StatementType::Invalid => false

    }
}    

