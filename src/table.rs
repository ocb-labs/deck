mod cell;
mod location;
mod value;
mod expression;

use std::collections::HashMap;

pub use cell::Cell;
pub use location::Location;
pub use value::Value;
pub use expression::Expression;

pub struct Table {
    cells: HashMap<Location, Cell>
}

impl Table {
    pub fn new() -> Self {
        Self {
            cells: HashMap::new()
        }
    }
}