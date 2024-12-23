use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, strum_macros::Display, strum_macros::EnumString)]
pub enum BasicLandType {
    Plains,
    Island,
    Swamp,
    Mountain,
    Forest,
}

#[derive(Debug, Clone, Serialize, Deserialize, strum_macros::Display, strum_macros::EnumString)]
pub enum NonBasicLandType {
    Cave,
    Desert,
    Gate,
    Lair,
    Locus,
    Sphere,
    Urzas,
}

#[derive(Debug, Clone, Serialize, Deserialize, strum_macros::Display)]
pub enum LandType {
    Basic(BasicLandType),
    #[strum(default)]
    NonBasic(NonBasicLandType),
}

impl FromStr for LandType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(nonbasic) = NonBasicLandType::from_str(s) {
            Ok(Self::NonBasic(nonbasic))
        } else if let Ok(basic) = BasicLandType::from_str(s) {
            Ok(Self::Basic(basic))
        } else {
            Err("Invalid land type".to_owned())
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LandData {
    #[serde(skip)]
    pub land_types: Vec<LandType>,
}
