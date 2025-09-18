use chrono::{DateTime, Utc};

pub enum Value {
    Whole(i64),
    Decimal(f64),
    Boolean(bool),
    Text(String),
    Date(DateTime<Utc>),
    Currency(f64),
    Blank
}