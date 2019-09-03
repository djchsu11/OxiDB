
pub struct Table {
    name: String,
    table: Vec<Row>
}

pub struct Row {
    row: Vec<Cell>
}

pub struct Cell{
    name: String,
    column_type: Type,
    value: Vec<u8>,
}

pub enum Type{
    INT,
    TEXT
}

impl Table{
    pub fn create_table(name: String, table: Vec<Row>) -> Option<Table>{
        for rows in table.iter(){
            for cell in rows.row.iter(){
                if cell.name == ""{
                    return None;
                }
            }
        }

        Some(Table{name, table})
    }
}