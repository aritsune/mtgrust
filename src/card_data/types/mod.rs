mod artifact_data;
mod battle_data;
mod creature_data;
mod enchantment_data;
mod instant_sorcery_data;
mod land_data;
mod planeswalker_data;
mod tribal_data;

use std::{fmt::Display, str::FromStr};

pub use artifact_data::{ArtifactData, ArtifactSubtype};
pub use battle_data::{BattleData, BattleType};
pub use creature_data::{CreatureData, CreatureType};
pub use enchantment_data::{EnchantmentData, EnchantmentType};
pub use instant_sorcery_data::{InstantData, SorceryData, SpellType};
pub use land_data::{BasicLandType, LandData, LandType, NonBasicLandType};
pub use planeswalker_data::{PlaneswalkerData, PlaneswalkerType};
pub use tribal_data::TribalData;

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
