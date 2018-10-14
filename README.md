# webmanifest
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Create a `manifest.webmanifest` file.

- [Documentation][8]
- [Crates.io][2]
- [Releases][releases]

## Examples
### Basic
```rust
extern crate webmanifest;
extern crate failure;

use webmanifest::{Manifest, Related};

fn main() -> Result<(), failure::Error> {
  let name = "My Cool Application";
  let url = "https://play.google.com/store/apps/details?id=cheeaun.hackerweb";
  let manifest = Manifest::builder(name)
    .short_name("my app")
    .background_color("#000")
    .related(&Related::new("play", url))
    .build()?;
  Ok(())
}
```

## Installation
```sh
$ cargo add webmanifest
```

## Safety
This crate uses ``#![deny(unsafe_code)]`` to ensure everything is implemented in
100% Safe Rust.

## Contributing
Want to join us? Check out our ["Contributing" guide][contributing] and take a
look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

## References
- https://developer.mozilla.org/en-US/docs/Web/Manifest

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/webmanifest.svg?style=flat-square
[2]: https://crates.io/crates/webmanifest
[3]: https://img.shields.io/travis/chooxide/webmanifest.svg?style=flat-square
[4]: https://travis-ci.org/chooxide/webmanifest
[5]: https://img.shields.io/crates/d/webmanifest.svg?style=flat-square
[6]: https://crates.io/crates/webmanifest
[7]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[8]: https://docs.rs/webmanifest

[releases]: https://github.com/chooxide/webmanifest/releases
[contributing]: https://github.com/chooxide/webmanifest/.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/chooxide/webmanifest/labels/good%20first%20issue
[help-wanted]: https://github.com/chooxide/webmanifest/labels/help%20wanted
