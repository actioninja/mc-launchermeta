use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct AssetIndex {
    pub objects: Vec<Object>,
    pub map_to_resources: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Object {
    pub hash: String,
    pub size: u64,
}
