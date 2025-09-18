mod expression;
mod value;

pub use expression::{Expression, Engine};
pub use value::Value;

pub struct Cell {
    value: Value,
    expression: Expression
}