use crate::{Instruction, Value};

pub struct Expression {
    cached_result: Value,
    instructions: Instruction
}

impl Expression {
    pub fn recalculate(&self) -> Value {
        Value::WholeNumber(0)
    }
}