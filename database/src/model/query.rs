use std::cmp::Ordering;

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
#[derive(Eq)]
pub struct Column {
    pub name: String,
    pub column_type: Type,
    pub column_value: Vec<u8>,
}
#[derive(Debug)]
#[derive(Eq)]
#[derive(PartialEq)]
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


impl Ord for Column{
    fn cmp(&self, other: &Self) -> Ordering{
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Column{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Column{
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
