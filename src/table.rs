use std::collections::HashMap;

use crate::value::Value;

#[derive(Hash, PartialEq, Eq)]
pub struct Location {
    column: u16,
    row: u64
}

pub struct Table {
    table: HashMap<Location, Value>
}

impl Table {
    fn new() -> Self {
        Self { table: HashMap::new() }
    }
}