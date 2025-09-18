mod cell;
mod location;

use std::collections::HashMap;

pub use cell::{Cell, Value, Expression, Engine};
pub use location::Location;

pub struct Table {
    data: HashMap<Location, Cell>
}

impl Table {
    pub fn new() -> Self {
        Self { data: HashMap::new() }
    }
}