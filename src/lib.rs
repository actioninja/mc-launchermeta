
////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2023. Rob Bailey                                              /
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

//! A simple crate that defines the types used by the Minecraft version manifest.
//!
//! This crate deliberately does not include any code to actually fetch from the endpoints, as to
//! not tie it to any particular HTTP client.

use serde::{Deserialize, Serialize};

pub mod version_manifest;
pub mod version;
pub mod asset_index;

/// The current URL to get the version manifest from.
pub const VERSION_MANIFEST_URL: &str = "https://launchermeta.mojang.com/mc/game/version_manifest.json";

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VersionKind {
    Release,
    Snapshot,
    OldBeta,
    OldAlpha,
    OldSnapshot,
    Experiment,
}
