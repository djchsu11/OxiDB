pub struct Table {
    name: String,
    table: Vec<Row>
}

pub struct Row {
    row: Vec<Cell>
}

pub struct Cell {
    name: String,
    column_type: Type,
    int_value: i32,
    text_value: String
}

pub enum Type{
    INT,
    TEXT
}

impl Cell{
    pub fn new(name: String, column_type: Type, int_value: i32, text_value: String) -> Cell{
        Cell{name, column_type, int_value, text_value}
    }
}

impl Table{
    pub fn new(name: String, row: Vec<Cell>) -> Table{
        let columns = Row{row};
        let table = vec![columns];
        Table{name, table}
    }
}