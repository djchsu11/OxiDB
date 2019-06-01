use std::collections::HashMap;

pub(crate) struct Table{
    rows:Vec<Row>,
    column_types:HashMap<String, ValidTypes>,
    table_name:String
}

pub(crate) struct Row{
    row:HashMap<String, Cell>
}

pub(crate) enum Cell{
    Int(i32),
    Text(String),
    Float(f64)
}

pub enum ValidTypes{
    Int,
    Text,
    Float
}

impl Table{
    pub fn test(){
        println!("Test");
    }
}
