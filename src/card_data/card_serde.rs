use serde::{de, Deserialize, Serialize};
use std::str::FromStr;
use strum::IntoEnumIterator;

use crate::card_data::{
    ArtifactSubtype, CreatureType, EnchantmentType, LandType, PlaneswalkerType,
};

use super::{
    types::{InstantData, SorceryData},
    ArtifactData, CardType, CardTypeData, CreatureData, EnchantmentData, LandData,
    PlaneswalkerData, TribalData,
};

impl From<FlatCardTypeData> for Vec<CardTypeData> {
    fn from(value: FlatCardTypeData) -> Self {
        CardType::iter()
            // TODO: figure out how to avoid cloning here
            .flat_map(|variant| match variant {
                CardType::Land => Some(CardTypeData::Land(value.clone().land_data?)),
                CardType::Creature => Some(CardTypeData::Creature(value.clone().creature_data?)),
                CardType::Artifact => Some(CardTypeData::Artifact(value.clone().artifact_data?)),
                CardType::Enchantment => {
                    Some(CardTypeData::Enchantment(value.clone().enchantment_data?))
                }
                CardType::Tribal => Some(CardTypeData::Tribal(value.clone().tribal_data?)),
                CardType::Planeswalker => {
                    Some(CardTypeData::Planeswalker(value.clone().planeswalker_data?))
                }
                CardType::Instant => None,
                CardType::Sorcery => None,
                CardType::Battle => None,
            })
            .collect::<Vec<_>>()
    }
}

#[derive(Serialize, Clone, Default)]
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
    #[serde(flatten)]
    instant_data: Option<InstantData>,
    #[serde(flatten)]
    sorcery_data: Option<SorceryData>,
}

impl From<Vec<CardTypeData>> for FlatCardTypeData {
    fn from(val: Vec<CardTypeData>) -> Self {
        let mut output = Self::default();
        macro_rules! extract_type_data {
            ($data:ident.$subtypes:ident -> $out:expr) => {{
                $out.extend(
                    $data
                        .$subtypes
                        .iter()
                        .map(|lt| lt.to_string())
                        .collect::<Vec<_>>(),
                )
            }};
        }
        for type_data in val {
            let name = CardType::from(&type_data).to_string();
            output.types.push(name);
            match type_data {
                CardTypeData::Land(data) => {
                    extract_type_data! { data.land_types -> output.subtypes }
                }
                CardTypeData::Creature(data) => {
                    extract_type_data! { data.creature_types -> output.subtypes }
                }
                CardTypeData::Artifact(data) => {
                    extract_type_data! { data.artifact_types -> output.subtypes }
                }
                CardTypeData::Enchantment(data) => {
                    extract_type_data! { data.enchantment_types -> output.subtypes }
                }
                CardTypeData::Tribal(data) => {
                    extract_type_data! { data.tribal_types -> output.subtypes }
                }
                CardTypeData::Planeswalker(data) => {
                    extract_type_data! { data.planeswalker_types -> output.subtypes }
                }
                CardTypeData::Instant(data) => {
                    extract_type_data! { data.spell_types -> output.subtypes }
                }
                CardTypeData::Sorcery(data) => {
                    extract_type_data! { data.spell_types -> output.subtypes }
                }
                CardTypeData::Battle => {
                    todo!()
                }
            };
        }
        output
    }
}

fn to_option<T, A, E, F: FnOnce(A) -> Result<T, E>>(f: F) -> impl FnOnce(A) -> Option<T> {
    |a: A| {
        let x = f(a);
        x.ok()
    }
}

impl<'de> Deserialize<'de> for FlatCardTypeData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use de::Error;
        use CardType::*;

        // TODO: generalize with visitor?
        let json: serde_json::value::Value = serde_json::value::Value::deserialize(deserializer)?;
        let type_values = json
            .get("types")
            .and_then(|t| t.as_array())
            .ok_or(Error::custom("types array not found"))?;

        let mut output = Self::default();
        let subtypes = json
            .get("subtypes")
            .and_then(|v| v.as_array())
            .ok_or(Error::custom("subtypes array not found"))?
            .iter()
            .map(|s| s.as_str().unwrap().to_owned())
            .collect::<Vec<_>>();
        let mut used_subtypes: Vec<&str> = vec![];
        // Macro for repetitive code to extract subtypes from type data
        macro_rules! de_type_data {
            ($datatype:ty:$name:ident.$subtypes:ident = $getter:expr) => {{
                let mut types: $datatype = serde_json::from_value(json.clone()).map_err(|e| {
                    Error::custom(format!(
                        "failed to deserialize {}: {}",
                        stringify!($datatype),
                        e,
                    ))
                })?;
                types.$subtypes = subtypes
                    .iter()
                    .flat_map(|i| {
                        if let Some(subtype) = $getter(i) {
                            used_subtypes.push(i);
                            Some(subtype)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                output.$name = Some(types);
            }};
        }
        for type_name in type_values {
            let card_type = type_name
                .as_str()
                .and_then(|v| CardType::from_str(v).ok())
                .ok_or(Error::custom("invalid type value"))?;
            output.types.push(card_type.to_string());
            match card_type {
                Land => {
                    de_type_data! { LandData: land_data.land_types = to_option(LandType::from_str) }
                }
                Creature => {
                    de_type_data! { CreatureData: creature_data.creature_types = CreatureType::new_validated }
                }
                Artifact => {
                    de_type_data! { ArtifactData: artifact_data.artifact_types = to_option(ArtifactSubtype::from_str) }
                }
                Enchantment => {
                    de_type_data! { EnchantmentData: enchantment_data.enchantment_types = to_option(EnchantmentType::from_str) }
                }
                Planeswalker => {
                    de_type_data! { PlaneswalkerData: planeswalker_data.planeswalker_types = PlaneswalkerType::new_validated }
                }
                Battle => todo!(),
                Tribal => todo!(),
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
