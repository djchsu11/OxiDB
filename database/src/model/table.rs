use std::collections::HashMap;
use std::cmp::Ordering;
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
#[derive(Eq)]
pub struct Cell {
    pub name: String,
    pub column_type: Type,
    pub value: Vec<u8>,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
pub enum Type {
    INT,
    TEXT,
    UNKNOWN,
}

impl Ord for Cell{
    fn cmp(&self, other: &Self) -> Ordering{
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Cell{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Cell{
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

