////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2023. Rob Bailey                                              /
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

//! Information about the libraries used by the game

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

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
    /// A list of artifacts to potentially download for the library
    pub downloads: Option<Downloads>,
    /// The name of the library, in the format `group:name:version`
    pub name: String,
    /// Information on how to extract the library.
    ///
    /// This is used for natives, and is a map of the files to extract to the directories to extract
    #[serde(default)]
    pub extract: Option<Extract>,
    /// Information on natives for the version
    ///
    /// This was used in older versions of the format
    #[serde(default)]
    pub natives: Option<Natives>,
    #[serde(default)]
    pub rules: Option<Vec<Rule>>,
}
