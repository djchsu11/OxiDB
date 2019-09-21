use std::collections::HashMap;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Debug)]
pub struct Database {
    pub registry: Vec<Table>,
}

#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub table: Vec<Row>,
}

#[derive(Debug)]
pub struct Row {
    pub row: Vec<Cell>,
}

#[derive(Debug)]
pub struct Cell {
    pub name: String,
    pub column_type: Type,
    pub value: Vec<u8>,
}

#[derive(Debug)]
pub enum Type {
    INT,
    TEXT,
    UNKNOWN,
}

impl Database {
    pub fn get_new_database() -> Database {
        Database {
            registry: Vec::new(),
        }
    }
}

//impl<'a> Deref for Database<'a>{
//    type Target = HashMap<String, Table>;
//
//    fn deref(&self) -> &Self::Target{
//        &self.registry
//    }
//}
//
//impl<'a> DerefMut for Database<'a> {
//    fn deref_mut(&mut self) -> &mut Self::Target {
//        self.registry
//    }
//}

impl Table {
    pub fn create_table(name: String, table: Vec<Row>) -> Option<Table> {
        for rows in table.iter() {
            for cell in rows.row.iter() {
                if cell.name == "".to_string() {
                    return None;
                }
            }
        }

        Some(Table { name, table })
    }
}
