
////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2023. Rob Bailey                                              /
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

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

impl Manifest {
    pub fn get_version(&self, id: &str) -> Option<&Version> {
        self.versions.iter().find(|v| v.id == id)
    }

    pub fn get_latest(&self, kind: VersionKind) -> Option<&Version> {
        match kind {
            VersionKind::Release => self.get_version(&self.latest.release),
            VersionKind::Snapshot => self.get_version(&self.latest.snapshot),
            _ => None,
        }
    }
}
