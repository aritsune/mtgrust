use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, strum_macros::Display, strum_macros::EnumString)]
pub enum BattleType {
    Siege,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BattleData {
    #[serde(skip)]
    pub battle_types: Vec<BattleType>,
}
