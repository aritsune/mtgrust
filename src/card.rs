use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr, FromInto};

use crate::mana::{Color, ManaCost, ManaSymbol};

#[derive(Debug, Serialize, Deserialize)]
pub enum CardSupertype {
    Basic,
    Legendary,
    Snow,
}

#[derive(Debug, Clone, Serialize, Deserialize, strum_macros::Display)]
pub enum LandType {
    Plains,
    Island,
    Swamp,
    Mountain,
    Forest,
    #[strum(default)]
    NonBasic(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreatureType(pub String);

impl Display for CreatureType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, strum_macros::Display)]
pub enum ArtifactSubtype {
    Equipment,
    #[strum(default)]
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, strum_macros::Display)]
pub enum EnchantmentSubtype {
    Aura,
    Saga,
    #[strum(default)]
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaneswalkerSubtype(pub String);

impl Display for PlaneswalkerSubtype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, strum_macros::Display)]
#[serde(untagged)]
pub enum CardTypeData {
    Land {
        #[serde(skip_serializing)]
        land_types: Vec<LandType>,
    },
    Creature {
        power: i32,
        toughness: i32,
        #[serde(skip_serializing)]
        creature_types: Vec<CreatureType>,
    },
    Instant,
    Sorcery,
    Artifact {
        #[serde(skip_serializing)]
        artifact_subtypes: Vec<ArtifactSubtype>,
    },
    Enchantment {
        #[serde(skip_serializing)]
        enchantment_subtypes: Vec<EnchantmentSubtype>,
    },
    Tribal {
        #[serde(skip_serializing)]
        tribal_types: Vec<CreatureType>,
    },
    Planeswalker {
        #[serde(skip_serializing)]
        planeswalker_subtypes: Vec<PlaneswalkerSubtype>,
        loyalty: usize,
    },
    Battle,
}

#[derive(Serialize, Deserialize, Clone)]
struct FlatCardTypeData {
    types: Vec<String>,
    subtypes: Vec<String>,
    #[serde(flatten)]
    land_data: Option<CardTypeData>,
    #[serde(flatten)]
    creature_data: Option<CardTypeData>,
    #[serde(flatten)]
    artifact_data: Option<CardTypeData>,
    #[serde(flatten)]
    enchantment_data: Option<CardTypeData>,
    #[serde(flatten)]
    tribal_data: Option<CardTypeData>,
    #[serde(flatten)]
    planeswalker_data: Option<CardTypeData>,
}

impl From<Vec<CardTypeData>> for FlatCardTypeData {
    fn from(val: Vec<CardTypeData>) -> Self {
        let mut output = Self {
            types: vec![],
            subtypes: vec![],
            land_data: None,
            creature_data: None,
            artifact_data: None,
            enchantment_data: None,
            tribal_data: None,
            planeswalker_data: None,
        };
        for type_data in val {
            let name = type_data.to_string();
            output.types.push(name);
            match type_data {
                CardTypeData::Land { ref land_types } => {
                    output.subtypes.extend(
                        land_types
                            .iter()
                            .map(|lt| lt.to_string())
                            .collect::<Vec<_>>(),
                    );
                    output.land_data = Some(type_data);
                }
                CardTypeData::Creature {
                    ref creature_types, ..
                } => {
                    output.subtypes.extend(
                        creature_types
                            .iter()
                            .map(|ct| ct.to_string())
                            .collect::<Vec<_>>(),
                    );
                    output.creature_data = Some(type_data);
                }
                //CardTypeData::Instant | CardTypeData::Sorcery => {}
                CardTypeData::Artifact {
                    ref artifact_subtypes,
                } => {
                    output.subtypes.extend(
                        artifact_subtypes
                            .iter()
                            .map(|at| at.to_string())
                            .collect::<Vec<_>>(),
                    );
                    output.artifact_data = Some(type_data);
                }
                CardTypeData::Enchantment {
                    ref enchantment_subtypes,
                } => {
                    output.subtypes.extend(
                        enchantment_subtypes
                            .iter()
                            .map(|et| et.to_string())
                            .collect::<Vec<_>>(),
                    );
                    output.enchantment_data = Some(type_data);
                }
                CardTypeData::Tribal { ref tribal_types } => {
                    output.subtypes.extend(
                        tribal_types
                            .iter()
                            .map(|tt| tt.to_string())
                            .collect::<Vec<_>>(),
                    );
                    output.tribal_data = Some(type_data);
                }
                CardTypeData::Planeswalker {
                    ref planeswalker_subtypes,
                    ..
                } => {
                    output.subtypes.extend(
                        planeswalker_subtypes
                            .iter()
                            .map(|pt| pt.to_string())
                            .collect::<Vec<_>>(),
                    );
                    output.planeswalker_data = Some(type_data);
                }
                CardTypeData::Instant | CardTypeData::Sorcery | CardTypeData::Battle => {}
            };
        }
        output
    }
}

#[serde_as]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub name: String,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub mana_cost: Option<ManaCost>,
    pub supertypes: Vec<CardSupertype>,
    #[serde_as(as = "FromInto<FlatCardTypeData>")]
    #[serde(flatten)]
    pub type_data: Vec<CardTypeData>,
}

pub fn test_serialize() -> Vec<String> {
    //let bear_de: Card = serde_json::from_str(
    //    r#"{"name":"Grizzly Bears","manaCost":"{1}{G}","supertypes":[],"types":["Creature"],"subtypes":["Bear"],"power":2,"toughness":2}"#,
    //).unwrap();
    //println!("{:?}", bear_de);

    let dryad = Card {
        name: "Dryad Arbor".to_owned(),
        mana_cost: None,
        supertypes: vec![],
        type_data: vec![
            CardTypeData::Creature {
                power: 1,
                toughness: 1,
                creature_types: vec![CreatureType("Dryad".to_owned())],
            },
            CardTypeData::Land {
                land_types: vec![LandType::Forest],
            },
        ],
    };
    let bears = Card {
        name: "Grizzly Bears".to_owned(),
        mana_cost: Some(ManaCost(vec![
            ManaSymbol::Generic(1),
            ManaSymbol::Colored(Color::Green),
        ])),
        supertypes: vec![],
        type_data: vec![CardTypeData::Creature {
            power: 2,
            toughness: 2,
            creature_types: vec![CreatureType("Bear".to_owned())],
        }],
    };
    vec![
        serde_json::to_string(&dryad).unwrap(),
        serde_json::to_string(&bears).unwrap(),
    ]
}
