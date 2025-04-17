use std::{fmt, str::FromStr};

use super::number::Number;

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Cell(Option<Number>);

impl Cell {
    pub(crate) fn empty() -> Self {
        Self(None)
    }

    pub(crate) fn filled(number: Number) -> Self {
        Self(Some(number))
    }

    pub(crate) fn value(&self) -> Option<u8> {
        self.0.as_ref().map(|number| number.value())
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Some(number) => write!(f, "{}", number),
            None => write!(f, " "),
        }
    }
}

impl FromStr for Cell {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.trim() {
            "" => Ok(Self::empty()),
            other => {
                let number = Number::from_str(other)?;
                Ok(Self::filled(number))
            }
        }
    }
}
