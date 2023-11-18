use serde::{Deserialize, Serialize};
use crate::VersionKind;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Latest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Version {
    pub id: String,
    pub url: String,
    pub time: String,
    pub release_time: String,
    #[serde(rename = "type")]
    pub kind: VersionKind,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub latest: Latest,
    pub versions: Vec<Version>,
}