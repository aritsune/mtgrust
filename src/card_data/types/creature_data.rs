use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::fmt::Display;

use super::MaybePlaceholderNumber;

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
