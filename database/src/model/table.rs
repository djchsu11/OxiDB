use std::collections::HashMap;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Debug)]
pub struct Table {
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
