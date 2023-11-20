
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
//!
//! ## Usage
//!
//! Basic usage of this crate would involve fetching the version manifest from the URL defined in
//! VERSION_MANIFEST_URL, and then fetching the version JSON file from the URL defined in the
//! Version.url field for the corresponding version.
//!
//! No examples are provided, as the exact usage will depend on the HTTP client used to fetch the
//! manifest and the version JSON files.
//!
//! ## Disclaimer
//!
//! This project is not affiliated with Minecraft, Mojang or Microsoft.
//!
//! All product and company names are trademarks™ or registered® trademarks of their respective
//! holders. Use of them does not imply any affiliation with or endorsement by them.
//!

use serde::{Deserialize, Serialize};

pub mod version_manifest;
pub mod version;
pub mod asset_index;

/// The current URL to get the version manifest from.
pub const VERSION_MANIFEST_URL: &str = "https://launchermeta.mojang.com/mc/game/version_manifest.json";

/// Type of Minecraft versions
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
