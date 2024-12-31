use crate::mana::ManaCost;
use enum_kinds::EnumKind;
use serde::{Deserialize, Serialize};

mod card_serde;
mod tests;
pub mod types;

use types::{
    ArtifactData, ArtifactSubtype, BattleData, CreatureData, CreatureType, EnchantmentData,
    EnchantmentType, InstantData, LandData, LandType, PlaneswalkerData, PlaneswalkerType,
    SorceryData, TribalData,
};

#[derive(Debug, Serialize, Deserialize)]
pub enum CardSupertype {
    Basic,
    Legendary,
    Snow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(EnumKind)]
#[enum_kind(
    CardType,
    derive(
        strum_macros::EnumIter,
        strum_macros::EnumString,
        strum_macros::Display
    )
)]
pub enum CardTypeData {
    Land(LandData),
    Creature(CreatureData),
    Instant(InstantData),
    Sorcery(SorceryData),
    Artifact(ArtifactData),
    Enchantment(EnchantmentData),
    Tribal(TribalData),
    Planeswalker(PlaneswalkerData),
    Battle(BattleData),
}

use card_serde::FlatCardTypeData;
use serde_with::{serde_as, DisplayFromStr, FromInto};
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardData {
    pub name: String,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub mana_cost: Option<ManaCost>,
    pub supertypes: Vec<CardSupertype>,
    #[serde_as(as = "FromInto<FlatCardTypeData>")]
    #[serde(flatten)]
    pub type_data: Vec<CardTypeData>,
}

pub fn test_serialize() -> Vec<String> {
    let bears: CardData = serde_json::from_value(serde_json::json!({
        "name": "Grizzly Bears",
        "manaCost":"{1}{G}",
        "supertypes":[],
        "types":["Creature"],
        "subtypes":["Bear"],
        "power":"2",
        "toughness":"2"
    }))
    .unwrap();
    println!("{:?}", bears);
    println!("{}", serde_json::to_string_pretty(&bears).unwrap());

    let dryad: CardData = serde_json::from_value(serde_json::json!({
        "name": "Dryad Arbor",
        "manaCost": null,
        "supertypes": [],
        "types": ["Creature","Land"],
        "subtypes": ["Forest","Dryad"],
        "power":"1",
        "toughness":"1",
    }))
    .unwrap();
    println!("{:?}", dryad);
    println!("{}", serde_json::to_string_pretty(&dryad).unwrap());

    vec![]
}
