use std::{fmt, str::FromStr};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Number(u8);

impl Number {
    pub(crate) fn new(value: u8) -> Option<Self> {
        if !(1..=9).contains(&value) {
            None
        } else {
            Some(Self(value))
        }
    }

    pub(crate) fn value(&self) -> u8 {
        self.0
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl FromStr for Number {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input_number = input
            .parse::<u8>()
            .map_err(|e| format!("Could not parse '{}' as a number: {}", input, e))?;
        if let Some(result) = Self::new(input_number) {
            Ok(result)
        } else {
            Err(format!("Value '{}' must be between 1 and 9.", input))
        }
    }
}
