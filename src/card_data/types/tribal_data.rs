use serde::{Deserialize, Serialize};

use super::CreatureType;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TribalData {
    #[serde(skip_serializing)]
    pub tribal_types: Vec<CreatureType>,
}
