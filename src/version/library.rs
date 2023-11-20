use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use crate::version::rule::Rule;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Artifact {
    pub path: String,
    pub sha1: String,
    pub size: u64,
    pub url: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Downloads {
    #[serde(default)]
    pub artifact: Option<Artifact>,
    #[serde(default)]
    pub classifiers: Option<BTreeMap<String, Artifact>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Natives {
    pub linux: Option<String>,
    pub osx: Option<String>,
    pub windows: Option<String>,
}

pub type Extract = BTreeMap<String, Vec<String>>;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Library {
    pub downloads: Option<Downloads>,
    pub name: String,
    #[serde(default)]
    pub extract: Option<Extract>,
    #[serde(default)]
    pub natives: Option<Natives>,
    #[serde(default)]
    pub rules: Option<Vec<Rule>>,
}
