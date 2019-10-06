#[derive(PartialEq, Debug)]
pub enum QueryType {
    Insert,
    Delete,
    Update,
    Select,
    Create,
    Invalid,
}

#[derive(Debug)]
pub struct Query {
    pub table_name: String,
    pub columns: Vec<Column>,
    pub operation: QueryType,
    pub all_columns_flag: bool,
}

#[derive(Debug)]
pub struct Column {
    pub name: String,
    pub column_type: Type,
    pub column_value: Vec<u8>,
}
#[derive(Debug)]
pub enum Type {
    INT,
    TEXT,
    UNKNOWN,
}

impl Query{
    pub fn new() -> Self{
        Query{ table_name: String::from(""), columns: Vec::new(), operation: QueryType::Invalid, all_columns_flag: false}
    }
}
