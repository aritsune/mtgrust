use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, strum_macros::Display, strum_macros::EnumString)]
pub enum SpellType {
    Adventure,
    Arcane,
    Lesson,
    Trap,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InstantData {
    pub spell_types: Vec<SpellType>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SorceryData {
    pub spell_types: Vec<SpellType>,
}
