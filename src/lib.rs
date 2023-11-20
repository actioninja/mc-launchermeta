use serde::{Deserialize, Serialize};

pub mod version_manifest;
pub mod version;
pub mod asset_index;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", serde_json::to_string(&VersionKind::Release));
        println!("{:?}", serde_json::to_string(&VersionKind::Snapshot));
        println!("{:?}", serde_json::to_string(&VersionKind::OldBeta));
        println!("{:?}", serde_json::to_string(&VersionKind::OldAlpha));
        println!("{:?}", serde_json::to_string(&VersionKind::OldSnapshot));
        println!("{:?}", serde_json::to_string(&VersionKind::Experiment));

        assert_eq!(4, 4);
    }
}
