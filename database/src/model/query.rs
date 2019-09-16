
#[derive(PartialEq)]
pub enum QueryType {
    Insert,
    Delete,
    Update,
    Select,
    Create,
    Invalid
}

pub struct Query{
    pub table_name: String,
    pub columns: Vec<Column>,
    pub operation: QueryType,
    pub all_columns_flag: bool    
}

pub struct Column{
    pub name: String,
    pub column_type: Type,
    pub column_value: Vec<u8>
    
}

pub enum Type{
    INT,
    TEXT,
    UNKNOWN
}

impl Column{
    pub fn new(name: String, column_type: Type, column_value: Vec<u8>) -> Column{
        Column{name, column_type, column_value}
    }
}

impl Query{
    pub fn new (table_name: String, columns: Vec<Column>, operation: QueryType, all_columns_flag: bool) -> Query{
        Query{table_name, columns, operation, all_columns_flag}
    }
}


