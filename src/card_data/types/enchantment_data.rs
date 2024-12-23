use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, strum_macros::Display, strum_macros::EnumString)]
pub enum EnchantmentType {
    Aura,
    Background,
    Cartouche,
    Case,
    Class,
    Curse,
    Role,
    Room,
    Rune,
    Saga,
    Shard,
    Shrine,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EnchantmentData {
    #[serde(skip)]
    pub enchantment_types: Vec<EnchantmentType>,
}
