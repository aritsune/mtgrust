use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

use super::MaybePlaceholderNumber;

static PLANESWALKER_TYPES: &str = include_str!("planeswalker_types.txt");

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaneswalkerType(pub String);

impl PlaneswalkerType {
    pub fn new_validated(input: &str) -> Option<Self> {
        if !PLANESWALKER_TYPES.lines().any(|l| l == input) {
            return None;
        }
        Some(Self(input.to_owned()))
    }
}

impl Display for PlaneswalkerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[serde_as]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PlaneswalkerData {
    #[serde(skip)]
    pub planeswalker_types: Vec<PlaneswalkerType>,
    #[serde_as(as = "DisplayFromStr")]
    pub loyalty: MaybePlaceholderNumber,
}
