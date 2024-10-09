use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Bitcoin(Decimal);

impl Display for Bitcoin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl From<Satochi> for Bitcoin {
    fn from(value: Satochi) -> Self {
        Self(Decimal::from(value.0) / dec!(100_000_000))
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Satochi(pub i64);

impl Display for Satochi {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl From<i64> for Satochi {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_conversions() {
        assert_eq!(
            Bitcoin::from(Satochi(36010516297)),
            Bitcoin(dec!(360.10516297))
        );
    }
}
