use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, strum_macros::Display, strum_macros::EnumString)]
pub enum ArtifactSubtype {
    Equipment,
    Fortification,
    Blood,
    Clue,
    Food,
    Gold,
    Incubator,
    Map,
    Treasure,
    Powerstone,
    Vehicle,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ArtifactData {
    #[serde(skip)]
    pub artifact_types: Vec<ArtifactSubtype>,
}
