pub mod currency;

use chrono::{DateTime, Utc};

use crate::value::currency::Currency;

pub enum Value {
    WholeNumber(i64),
    DecimalNumber(f64),
    Boolean(bool),
    Text(String),
    Date(DateTime<Utc>),
    Currency(Currency),
    Blank
}