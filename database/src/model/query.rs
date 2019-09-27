#[derive(PartialEq)]
pub enum QueryType {
    Insert,
    Delete,
    Update,
    Select,
    Create,
    Invalid,
}

pub struct Query {
    pub table_name: String,
    pub columns: Vec<Column>,
    pub operation: QueryType,
    pub all_columns_flag: bool,
}

pub struct Column {
    pub name: String,
    pub column_type: Type,
    pub column_value: Vec<u8>,
}

pub enum Type {
    INT,
    TEXT,
    UNKNOWN,
}
