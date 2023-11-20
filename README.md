# mc-launchermeta

<!-- cargo-rdme start -->

A simple crate that defines the types used by the Minecraft version manifest.

This crate deliberately does not include any code to actually fetch from the endpoints, as to
not tie it to any particular HTTP client.

### Usage

Basic usage of this crate would involve fetching the version manifest from the URL defined in
VERSION_MANIFEST_URL, and then fetching the version JSON file from the URL defined in the
Version.url field for the corresponding version.

No examples are provided, as the exact usage will depend on the HTTP client used to fetch the
manifest and the version JSON files.

### Disclaimer

This project is not affiliated with Minecraft, Mojang or Microsoft.

All product and company names are trademarks™ or registered® trademarks of their respective
holders. Use of them does not imply any affiliation with or endorsement by them.

<!-- cargo-rdme end -->

## License

This project is licensed under the MPL-2.0 license. See the [LICENSE.md](./LICENSE.md) file for
more info.
