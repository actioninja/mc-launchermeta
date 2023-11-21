use mc_launchermeta::version::Version;
use mc_launchermeta::version_manifest::Manifest;
use mc_launchermeta::VERSION_MANIFEST_URL;

#[test]
fn fetch_version_manifest() {
    let _ = reqwest::blocking::get(VERSION_MANIFEST_URL)
        .unwrap()
        .json::<Manifest>()
        .unwrap();
}

#[test]
fn test_all_versions() {
    let version_manifest = reqwest::blocking::get(VERSION_MANIFEST_URL)
        .unwrap()
        .json::<Manifest>()
        .unwrap();
    for version in version_manifest.versions {
        let _ = reqwest::blocking::get(&version.url)
            .unwrap()
            .json::<Version>()
            .unwrap();
    }
}
