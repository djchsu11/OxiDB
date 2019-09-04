
pub struct Table {
    pub name: String,
    pub table: Vec<Row>
}

pub struct Row {
    pub row: Vec<Cell>
}

pub struct Cell{
    pub name: String,
    pub column_type: Type,
    pub value: Vec<u8>,
}

pub enum Type{
    INT,
    TEXT,
    UNKNOWN
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