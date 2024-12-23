use crate::mana::ManaCost;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr, FromInto};

mod card_serde;
mod types;
use card_serde::FlatCardTypeData;

pub use types::artifact_data::{ArtifactData, ArtifactSubtype};
pub use types::creature_data::{CreatureData, CreatureType};
pub use types::enchantment_data::{EnchantmentData, EnchantmentType};
pub use types::land_data::{LandData, LandType};
pub use types::planeswalker_data::{PlaneswalkerData, PlaneswalkerType};
pub use types::tribal_data::TribalData;

#[derive(Debug, Serialize, Deserialize)]
pub enum CardSupertype {
    Basic,
    Legendary,
    Snow,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    strum_macros::Display,
    strum_macros::EnumString,
    strum_macros::EnumIter,
)]
#[serde(untagged)]
pub enum CardTypeData {
    Land(LandData),
    Creature(CreatureData),
    Instant,
    Sorcery,
    Artifact(ArtifactData),
    Enchantment(EnchantmentData),
    Tribal(TribalData),
    Planeswalker(PlaneswalkerData),
    Battle,
}

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
