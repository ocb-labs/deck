use crate::{Expression, table::Value};

pub enum Either<A, B> {
    This(A),
    That(B)
}

pub struct Cell {
    pub value: Value,
    expression: Option<Expression>,
    to_call: Vec<fn () -> ()>
}

impl Cell {
    pub fn from_value(value: Value) -> Self {
        Self { value: value, expression: None, to_call: Vec::new() }
    }

    pub fn from_expr(expression: Expression) -> Self {
        Self { value: expression.recalculate(), expression: Some(expression), to_call: Vec::new() }
    }

    pub fn notify_update(&mut self, callback: fn () -> ()) {
        self.to_call.push(callback);
    }

    pub fn update(&mut self, object: Either<Value, Expression>) {
        self.value = match object {
            Either::This(value) => value,
            Either::That(expr) => expr.recalculate()
        };

        for caller in self.to_call.clone() {
            caller();
        }
    }
}