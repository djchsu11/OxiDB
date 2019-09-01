
pub struct Query{
    table_name: String,
    columns: Vec<Column>,
    operation: StatementType,
    all_columns_flag: bool    
}

pub struct Column{
    name: String,
    column_type: Type,
    
}

pub enum Type{
    INT,
    TEXT,
    UNKNOWN
}

#[derive(PartialEq)]
pub enum StatementType {
    Insert,
    Delete,
    Update,
    Select,
    Create,
    Invalid
}

impl Column{
    pub fn new(name: String, column_type: Type) -> Column{
        Column{name, column_type}
    }
}

impl Query{
    pub fn new (table_name: String, columns: Vec<Column>, operation: StatementType, all_columns_flag: bool) -> Query{
        Query{table_name, columns, operation, all_columns_flag}
    }
}