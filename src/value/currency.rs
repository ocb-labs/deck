use std::ops::{Add, Sub, Mul, Div};

const SCALE: i64 = 10_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Currency(i64);

impl Currency {
    fn new(value: f64) -> Option<Self> {
        let scaled = (value * SCALE as f64).round();
        if scaled < i64::MIN as f64 || scaled > i64::MAX as f64 {
            None
        } else {
            Some(Self(scaled as i64))
        }
    }

    fn to_f64(self) -> f64 {
        self.0 as f64 / SCALE as f64
    }
}

impl Add for Currency {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Currency {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul for Currency {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self((self.0 * rhs.0) / SCALE)
    }
}

impl Div for Currency {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self((self.0 * SCALE) / rhs.0)
    }
}
