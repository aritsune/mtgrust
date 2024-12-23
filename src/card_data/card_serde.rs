use rand::seq::SliceRandom;
use serde::{de, Deserialize, Serialize};
use std::{fs, str::FromStr};
use strum::IntoEnumIterator;

use crate::card_data::{
    ArtifactSubtype, CreatureType, EnchantmentType, LandType, PlaneswalkerType,
};

use super::{
    ArtifactData, CardData, CardTypeData, CreatureData, EnchantmentData, LandData,
    PlaneswalkerData, TribalData,
};

impl From<FlatCardTypeData> for Vec<CardTypeData> {
    fn from(value: FlatCardTypeData) -> Self {
        CardTypeData::iter()
            // TODO: figure out how to avoid cloning here
            .flat_map(|variant| match variant {
                CardTypeData::Land { .. } => Some(CardTypeData::Land(value.clone().land_data?)),
                CardTypeData::Creature(..) => {
                    Some(CardTypeData::Creature(value.clone().creature_data?))
                }
                CardTypeData::Artifact { .. } => {
                    Some(CardTypeData::Artifact(value.clone().artifact_data?))
                }
                CardTypeData::Enchantment { .. } => {
                    Some(CardTypeData::Enchantment(value.clone().enchantment_data?))
                }
                CardTypeData::Tribal { .. } => {
                    Some(CardTypeData::Tribal(value.clone().tribal_data?))
                }
                CardTypeData::Planeswalker { .. } => {
                    Some(CardTypeData::Planeswalker(value.clone().planeswalker_data?))
                }
                CardTypeData::Instant => None,
                CardTypeData::Sorcery => None,
                CardTypeData::Battle => None,
            })
            .collect::<Vec<_>>()
    }
}

#[derive(Serialize, Clone)]
pub struct FlatCardTypeData {
    types: Vec<String>,
    subtypes: Vec<String>,
    #[serde(flatten)]
    land_data: Option<LandData>,
    #[serde(flatten)]
    creature_data: Option<CreatureData>,
    #[serde(flatten)]
    artifact_data: Option<ArtifactData>,
    #[serde(flatten)]
    enchantment_data: Option<EnchantmentData>,
    #[serde(flatten)]
    tribal_data: Option<TribalData>,
    #[serde(flatten)]
    planeswalker_data: Option<PlaneswalkerData>,
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
                CardTypeData::Land(land_data) => {
                    output.subtypes.extend(
                        land_data
                            .land_types
                            .iter()
                            .map(|lt| lt.to_string())
                            .collect::<Vec<_>>(),
                    );
                    output.land_data = Some(land_data);
                }
                CardTypeData::Creature(creature_data) => {
                    output.subtypes.extend(
                        creature_data
                            .creature_types
                            .iter()
                            .map(|ct| ct.to_string())
                            .collect::<Vec<_>>(),
                    );
                    output.creature_data = Some(creature_data);
                }
                //CardTypeData::Instant | CardTypeData::Sorcery => {}
                CardTypeData::Artifact(artifact_data) => {
                    output.subtypes.extend(
                        artifact_data
                            .artifact_types
                            .iter()
                            .map(|at| at.to_string())
                            .collect::<Vec<_>>(),
                    );
                    output.artifact_data = Some(artifact_data);
                }
                CardTypeData::Enchantment(enchantment_data) => {
                    output.subtypes.extend(
                        enchantment_data
                            .enchantment_types
                            .iter()
                            .map(|et| et.to_string())
                            .collect::<Vec<_>>(),
                    );
                    output.enchantment_data = Some(enchantment_data);
                }
                CardTypeData::Tribal(tribal_data) => {
                    output.subtypes.extend(
                        tribal_data
                            .tribal_types
                            .iter()
                            .map(|tt| tt.to_string())
                            .collect::<Vec<_>>(),
                    );
                    output.tribal_data = Some(tribal_data);
                }
                CardTypeData::Planeswalker(planeswalker_data) => {
                    output.subtypes.extend(
                        planeswalker_data
                            .planeswalker_types
                            .iter()
                            .map(|pt| pt.to_string())
                            .collect::<Vec<_>>(),
                    );
                    output.planeswalker_data = Some(planeswalker_data);
                }
                CardTypeData::Instant | CardTypeData::Sorcery | CardTypeData::Battle => {}
            };
        }
        output
    }
}

impl<'de> Deserialize<'de> for FlatCardTypeData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use de::Error;
        use CardTypeData::*;

        // todo: generalize with visitor?
        let json: serde_json::value::Value = serde_json::value::Value::deserialize(deserializer)?;
        let type_values = json
            .get("types")
            .and_then(|t| t.as_array())
            .ok_or(Error::custom("types array not found"))?;

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
        let subtypes = json
            .get("subtypes")
            .and_then(|v| v.as_array())
            .ok_or(Error::custom("subtypes array not found"))?
            .iter()
            .map(|s| s.as_str().unwrap().to_owned())
            .collect::<Vec<_>>();
        let mut used_subtypes: Vec<&str> = vec![];
        for type_name in type_values {
            let card_type: CardTypeData = type_name
                .as_str()
                .and_then(|v| CardTypeData::from_str(v).ok())
                .ok_or(Error::custom("invalid type value"))?;
            output.types.push(card_type.to_string());
            match card_type {
                Land { .. } => {
                    let mut land_data: LandData =
                        serde_json::from_value(json.clone()).map_err(|e| {
                            Error::custom(format!("failed to deserialize land data: {}", e))
                        })?;
                    land_data.land_types = subtypes
                        .iter()
                        .flat_map(|i| {
                            if let Ok(ty) = LandType::from_str(i) {
                                used_subtypes.push(i);
                                Some(ty)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();
                    output.land_data = Some(land_data);
                }
                Creature { .. } => {
                    let mut creature_data: CreatureData = serde_json::from_value(json.clone())
                        .map_err(|e| {
                            Error::custom(format!("failed to deserialize creature data: {}", e))
                        })?;
                    let creature_types = subtypes
                        .iter()
                        .flat_map(|i| {
                            if let Some(ty) = CreatureType::new_validated(i) {
                                used_subtypes.push(i);
                                Some(ty)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();
                    creature_data.creature_types = creature_types;
                    output.creature_data = Some(creature_data);
                }
                Artifact { .. } => {
                    let mut artifact_data: ArtifactData = serde_json::from_value(json.clone())
                        .map_err(|e| {
                            Error::custom(format!("failed to deserialize artifact data: {}", e))
                        })?;
                    let artifact_types = subtypes
                        .iter()
                        .flat_map(|i| {
                            if let Ok(ty) = ArtifactSubtype::from_str(i) {
                                used_subtypes.push(i);
                                Some(ty)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();
                    artifact_data.artifact_types = artifact_types;
                    output.artifact_data = Some(artifact_data);
                }
                Enchantment { .. } => {
                    let mut enchantment_data: EnchantmentData =
                        serde_json::from_value(json.clone()).map_err(|e| {
                            Error::custom(format!("failed to deserialize enchantment data: {}", e))
                        })?;
                    let enchantment_types = subtypes
                        .iter()
                        .flat_map(|i| {
                            if let Ok(ty) = EnchantmentType::from_str(i) {
                                used_subtypes.push(i);
                                Some(ty)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();
                    enchantment_data.enchantment_types = enchantment_types;
                    output.enchantment_data = Some(enchantment_data);
                }
                Planeswalker(..) => {
                    let mut planeswalker_data: PlaneswalkerData =
                        serde_json::from_value(json.clone()).map_err(|e| {
                            Error::custom(format!("failed to deserialize planeswalker data: {}", e))
                        })?;
                    let planeswalker_types = subtypes
                        .iter()
                        .flat_map(|i| {
                            if let Some(ty) = PlaneswalkerType::new_validated(i) {
                                used_subtypes.push(i);
                                Some(ty)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();
                    planeswalker_data.planeswalker_types = planeswalker_types;
                    output.planeswalker_data = Some(planeswalker_data);
                }
                Battle => todo!(),
                Tribal(..) => todo!(),
                Instant | Sorcery => {}
            }
        }

        let unused_subtypes = subtypes
            .iter()
            .filter(|st| !used_subtypes.contains(&st.as_str()))
            .collect::<Vec<_>>();

        if !unused_subtypes.is_empty() {
            return Err(Error::custom(format!(
                "Invalid subtypes: {:?}",
                unused_subtypes
            )));
        }

        Ok(output)
    }
}

#[derive(Debug, Deserialize)]
struct MtgJsonData {
    cards: Vec<CardData>,
}
#[derive(Debug, Deserialize)]
struct MtgJsonSet {
    data: MtgJsonData,
}

#[test]
fn deserialize_alpha() {
    let file =
        serde_json::de::from_str::<MtgJsonSet>(&fs::read_to_string("./data/LEA.json").unwrap())
            .unwrap();
    //println!(
    //    "{:?}",
    //    file.data.cards.choose_multiple(&mut rand::thread_rng(), 3)
    //)
}

#[test]
fn de_neo() {
    let file =
        serde_json::de::from_str::<MtgJsonSet>(&fs::read_to_string("./data/NEO.json").unwrap())
            .unwrap();
    println!(
        "{:#?}",
        file.data.cards.choose_multiple(&mut rand::thread_rng(), 3)
    )
}

#[test]
fn reconfigure() {
    let card: CardData = serde_json::from_value(
        serde_json::json!({"object":"card","id":"5d33a5b7-797b-4079-8d62-edd124c0fb5a","oracle_id":"c739e180-2f14-41ed-8e7e-50b7df985f35","multiverse_ids":[548461],"mtgo_id":97246,"arena_id":79588,"tcgplayer_id":262809,"cardmarket_id":608244,"name":"Rabbit Battery","lang":"en","released_at":"2022-02-18","uri":"https://api.scryfall.com/cards/5d33a5b7-797b-4079-8d62-edd124c0fb5a","scryfall_uri":"https://scryfall.com/card/neo/157/rabbit-battery?utm_source=api","layout":"normal","highres_image":true,"image_status":"highres_scan","image_uris":{"small":"https://cards.scryfall.io/small/front/5/d/5d33a5b7-797b-4079-8d62-edd124c0fb5a.jpg?1654567784","normal":"https://cards.scryfall.io/normal/front/5/d/5d33a5b7-797b-4079-8d62-edd124c0fb5a.jpg?1654567784","large":"https://cards.scryfall.io/large/front/5/d/5d33a5b7-797b-4079-8d62-edd124c0fb5a.jpg?1654567784","png":"https://cards.scryfall.io/png/front/5/d/5d33a5b7-797b-4079-8d62-edd124c0fb5a.png?1654567784","art_crop":"https://cards.scryfall.io/art_crop/front/5/d/5d33a5b7-797b-4079-8d62-edd124c0fb5a.jpg?1654567784","border_crop":"https://cards.scryfall.io/border_crop/front/5/d/5d33a5b7-797b-4079-8d62-edd124c0fb5a.jpg?1654567784"},"mana_cost":"{R}","cmc":1.0,
            "supertypes": [], "types": ["Artifact","Creature"], "subtypes": ["Rabbit","Equipment"]
            ,"oracle_text":"Haste\nEquipped creature gets +1/+1 and has haste.\nReconfigure {R} ({R}: Attach to target creature you control; or unattach from a creature. Reconfigure only as a sorcery. While attached, this isn't a creature.)","power":"1","toughness":"1","colors":["R"],"color_identity":["R"],"keywords":["Haste","Reconfigure"]}),
    ).unwrap();
    println!("{:#?}", card);
}
