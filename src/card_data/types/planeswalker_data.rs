use std::fmt::Display;

use serde::{Deserialize, Serialize};

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

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaneswalkerData {
    #[serde(skip_serializing)]
    pub planeswalker_types: Vec<PlaneswalkerType>,
    pub loyalty: usize,
}
