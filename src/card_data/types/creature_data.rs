use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::{
    fmt::{Display, Write},
    str::FromStr,
};

static CREATURE_TYPES: &str = include_str!("creature_types.txt");

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreatureType(pub String);

impl CreatureType {
    pub fn new_validated(input: &str) -> Option<Self> {
        if !CREATURE_TYPES.lines().any(|l| l == input) {
            return None;
        }
        Some(Self(input.to_owned()))
    }
}

impl Display for CreatureType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MaybePlaceholderNumber {
    Placeholder,
    Number(i32),
}

impl Display for MaybePlaceholderNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Placeholder => f.write_str("*"),
            Self::Number(i) => f.write_fmt(format_args!("{}", i)),
        }
    }
}

impl FromStr for MaybePlaceholderNumber {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Self::Placeholder),
            other => Ok(Self::Number(
                str::parse(other).map_err(|_| "Failed to convert to number")?,
            )),
        }
    }
}

impl Default for MaybePlaceholderNumber {
    fn default() -> Self {
        Self::Number(0)
    }
}

#[serde_as]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreatureData {
    #[serde_as(as = "DisplayFromStr")]
    pub power: MaybePlaceholderNumber,
    #[serde_as(as = "DisplayFromStr")]
    pub toughness: MaybePlaceholderNumber,
    #[serde(skip)]
    pub creature_types: Vec<CreatureType>,
}
