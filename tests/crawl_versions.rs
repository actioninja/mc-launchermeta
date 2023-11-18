use serde_json::Value;
use mc_launchermeta::version::Version;
use mc_launchermeta::version_manifest::Manifest;

const VERSION_MANIFEST_URL: &str = "https://launchermeta.mojang.com/mc/game/version_manifest.json";

#[test]
fn fetch_version_manifest() {
    let version_manifest = reqwest::blocking::get(VERSION_MANIFEST_URL).unwrap().json::<Manifest>().unwrap();
    println!("{:#?}", version_manifest);
}

#[test]
fn dump_version_manifest() {
    let version_manifest = reqwest::blocking::get(VERSION_MANIFEST_URL).unwrap().json::<Manifest>().unwrap();
    let version_manifest_json = serde_json::to_string_pretty(&version_manifest).unwrap();
    std::fs::write("tests/data/version_manifest.json", version_manifest_json).unwrap();

    for version in version_manifest.versions {
        let version_json: Value = reqwest::blocking::get(&version.url).unwrap().json().unwrap();
        let version_json = serde_json::to_string_pretty(&version_json).unwrap();
        std::fs::write(format!("tests/data/versions/{}.json", version.id), version_json).unwrap();
    }
}

#[test]
/// walk through all versions downloaded and attempt to parse them
fn test_all_versions() {
    let versions = std::fs::read_dir("tests/data/versions").unwrap();
    for version in versions {
        println!("{:?}", version);
        let version = version.unwrap();
        let version_json: Version = serde_json::from_str(&std::fs::read_to_string(version.path()).unwrap()).unwrap();
    }
}
