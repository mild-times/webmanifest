## 2018-10-16, Version 1.0.3
### Commits
- [[`aada7660d2`](https://github.com/chooxide/webmanifest/commit/aada7660d2d32834382bf3ea8b15687221475fbc)] (cargo-release) version 1.0.3 (Yoshua Wuyts)
- [[`1687d8a87b`](https://github.com/chooxide/webmanifest/commit/1687d8a87bc78819c77d2ca5896857f676dbc0a5)] fix typo in enum (Yoshua Wuyts)
- [[`e4fcb06b97`](https://github.com/chooxide/webmanifest/commit/e4fcb06b974802a7719c329b13416035c22c21b7)] Update changelog (Yoshua Wuyts)

### Stats
```diff
 CHANGELOG.md       | 15 +++++++++++++++
 Cargo.toml         |  2 +-
 src/orientation.rs |  2 +-
 3 files changed, 17 insertions(+), 2 deletions(-)
```


## 2018-10-16, Version 1.0.2
### Commits
- [[`c9075e2a87`](https://github.com/chooxide/webmanifest/commit/c9075e2a8704b782a8d92164df593415a93608da)] (cargo-release) version 1.0.2 (Yoshua Wuyts)
- [[`f45de900a3`](https://github.com/chooxide/webmanifest/commit/f45de900a3684e4c649b921d8176ace125e3b0fb)] finish documentation orientations (Yoshua Wuyts)
- [[`461b656751`](https://github.com/chooxide/webmanifest/commit/461b656751e51c4e5485abb23f09d20e38ad6bd9)] Update changelog (Yoshua Wuyts)

### Stats
```diff
 CHANGELOG.md       | 24 ++++++++++++++++++++++++
 Cargo.toml         |  2 +-
 src/orientation.rs | 28 ++++++++++++++++++----------
 3 files changed, 43 insertions(+), 11 deletions(-)
```


## 2018-10-16, Version 1.0.1
### Commits
- [[`7e8a98130a`](https://github.com/chooxide/webmanifest/commit/7e8a98130a2f891ec6db3dbd9365286a1670211a)] (cargo-release) version 1.0.1 (Yoshua Wuyts)
- [[`996378e846`](https://github.com/chooxide/webmanifest/commit/996378e84651bed5e10ef5ba110aba0044b943a0)] improve arg docs (Yoshua Wuyts)
- [[`c3ec25e02a`](https://github.com/chooxide/webmanifest/commit/c3ec25e02a5f1cd44fda699f6cbbb1fb071d34b3)] templates (Yoshua Wuyts)
- [[`ce07f6b77c`](https://github.com/chooxide/webmanifest/commit/ce07f6b77cca48bad126cba562555aebeb992f3f)] document example (Yoshua Wuyts)
- [[`87b8864812`](https://github.com/chooxide/webmanifest/commit/87b8864812e21e3e680ecd72550f8af10a0ab1ba)] clippy & rustfmt (Yoshua Wuyts)
- [[`0951293290`](https://github.com/chooxide/webmanifest/commit/095129329074ef9a4f4f9eb91b87c4a1526975a3)] Update changelog (Yoshua Wuyts)

### Stats
```diff
 .github/ISSUE_TEMPLATE.md                 | 40 +------------------
 .github/ISSUE_TEMPLATE/bug_report.md      | 23 +++++++++++-
 .github/ISSUE_TEMPLATE/feature_request.md | 43 +++++++++++++++++++++-
 .github/ISSUE_TEMPLATE/question.md        | 18 +++++++++-
 .travis.yml                               |  4 +-
 CHANGELOG.md                              | 25 ++++++++++++-
 Cargo.toml                                |  2 +-
 README.md                                 |  2 +-
 src/lib.rs                                | 66 ++++++++++++++++----------------
 9 files changed, 150 insertions(+), 73 deletions(-)
```


## 2018-10-14, Version 1.0.0
### Commits
- [[`1b180cdb10`](https://github.com/chooxide/webmanifest/commit/1b180cdb10bb853ddea33aca7997c7739e26fdc1)] (cargo-release) version 1.0.0 (Yoshua Wuyts)
- [[`0e6deae4c1`](https://github.com/chooxide/webmanifest/commit/0e6deae4c11a163e70c5f85949c355ca215f54f1)] finish up last examples (Yoshua Wuyts)
- [[`f228d5e29b`](https://github.com/chooxide/webmanifest/commit/f228d5e29b7afbd68acd2b8f836416c213557df6)] scope and more (Yoshua Wuyts)
- [[`4d08a9d3cd`](https://github.com/chooxide/webmanifest/commit/4d08a9d3cd7ff52d00b8728ba1f4f2a1d6a6c182)] even more APIs (Yoshua Wuyts)
- [[`bd2f790ed1`](https://github.com/chooxide/webmanifest/commit/bd2f790ed131ca19732c02af2b0ede59580e912f)] more APIs (Yoshua Wuyts)
- [[`4cfaa93866`](https://github.com/chooxide/webmanifest/commit/4cfaa9386687002a0b6414de987ae5aa83bab121)] split structs (Yoshua Wuyts)
- [[`d86dacbdfc`](https://github.com/chooxide/webmanifest/commit/d86dacbdfc6a4a2ed8dfc95b9baebfadfd5ad422)] Update changelog (Yoshua Wuyts)

### Stats
```diff
 CHANGELOG.md        |  15 +++-
 Cargo.toml          |   2 +-
 src/direction.rs    |  30 +++++-
 src/display_mode.rs |  36 ++++++-
 src/error.rs        |  80 +--------------
 src/icon.rs         |  61 ++++++++++-
 src/lib.rs          | 311 +++++++++++++++++++++++++++++++----------------------
 src/orientation.rs  |  42 +++++++-
 src/related.rs      |  38 ++++++-
 9 files changed, 407 insertions(+), 208 deletions(-)
```


## 2018-10-14, Version 0.2.5
### Commits
- [[`ff92a88ca0`](https://github.com/chooxide/webmanifest/commit/ff92a88ca01d8e32bb893cf9694911c988dff22d)] (cargo-release) version 0.2.5 (Yoshua Wuyts)
- [[`48f204dc1a`](https://github.com/chooxide/webmanifest/commit/48f204dc1a1e44505a864b163267e3c120775ced)] strip null (Yoshua Wuyts)
- [[`3ff0b63a5a`](https://github.com/chooxide/webmanifest/commit/3ff0b63a5aba5668685c0fd324837b5e5e5fbbba)] Update changelog (Yoshua Wuyts)

### Stats
```diff
 CHANGELOG.md | 15 +++++++++++++++
 Cargo.toml   |  2 +-
 src/lib.rs   | 12 +++++++++---
 3 files changed, 25 insertions(+), 4 deletions(-)
```


## 2018-10-14, Version 0.2.4
### Commits
- [[`c86abab6e1`](https://github.com/chooxide/webmanifest/commit/c86abab6e101f9577717f030d7bd319cb833e51f)] (cargo-release) version 0.2.4 (Yoshua Wuyts)
- [[`cd1e4231f5`](https://github.com/chooxide/webmanifest/commit/cd1e4231f5a02c13a20e10bcb41ea1eec2129648)] simplify the mime guess logic (Yoshua Wuyts)
- [[`c668c3836e`](https://github.com/chooxide/webmanifest/commit/c668c3836e07fb668e2f09e9d0926c78ef60e04f)] Update changelog (Yoshua Wuyts)

### Stats
```diff
 CHANGELOG.md | 15 +++++++++++++++
 Cargo.toml   |  2 +-
 src/lib.rs   |  9 +--------
 3 files changed, 17 insertions(+), 9 deletions(-)
```


## 2018-10-14, Version 0.2.3
### Commits
- [[`670ae0f408`](https://github.com/chooxide/webmanifest/commit/670ae0f4083fad7069170e7d1cadf105fbe1de98)] (cargo-release) version 0.2.3 (Yoshua Wuyts)
- [[`3d43169fb8`](https://github.com/chooxide/webmanifest/commit/3d43169fb8f924c49fa33a834bece95684f392ba)] default display mode (Yoshua Wuyts)
- [[`82b8023b35`](https://github.com/chooxide/webmanifest/commit/82b8023b3533ff515b04a355c694c0f45fcd910d)] Update changelog (Yoshua Wuyts)

### Stats
```diff
 CHANGELOG.md | 15 +++++++++++++++
 Cargo.toml   |  2 +-
 src/lib.rs   |  6 +++---
 3 files changed, 19 insertions(+), 4 deletions(-)
```


## 2018-10-14, Version 0.2.2
### Commits
- [[`beb35e1dd6`](https://github.com/chooxide/webmanifest/commit/beb35e1dd6edf1e6cd9415a751e5872bb47e8ef9)] (cargo-release) version 0.2.2 (Yoshua Wuyts)
- [[`9827f166bc`](https://github.com/chooxide/webmanifest/commit/9827f166bc5702f94e8d62aafd91eb2b04f88163)] theme color (Yoshua Wuyts)
- [[`e071be5922`](https://github.com/chooxide/webmanifest/commit/e071be5922c606da1a85ff61e65619fa61999675)] Update changelog (Yoshua Wuyts)

### Stats
```diff
 CHANGELOG.md | 15 +++++++++++++++
 Cargo.toml   |  2 +-
 src/lib.rs   | 23 +++++++++++++++++++++++
 3 files changed, 39 insertions(+), 1 deletion(-)
```


## 2018-10-14, Version 0.2.1
### Commits
- [[`3fc39b72dc`](https://github.com/chooxide/webmanifest/commit/3fc39b72dccfae5edcbca2ff1076968774951e45)] (cargo-release) version 0.2.1 (Yoshua Wuyts)
- [[`4076ff26de`](https://github.com/chooxide/webmanifest/commit/4076ff26de15d3f0b9a37896835e71c1e8564355)] fix check (Yoshua Wuyts)
- [[`4a020dc5db`](https://github.com/chooxide/webmanifest/commit/4a020dc5db5882598d11ebb2d86996400f087a3c)] Update changelog (Yoshua Wuyts)

### Stats
```diff
 CHANGELOG.md | 17 +++++++++++++++++
 Cargo.toml   |  2 +-
 src/lib.rs   |  2 +-
 3 files changed, 19 insertions(+), 2 deletions(-)
```


## 2018-10-14, Version 0.2.0
### Commits
- [[`c17041743d`](https://github.com/chooxide/webmanifest/commit/c17041743d4d72042df0acba1fb7e71c02661fb4)] (cargo-release) version 0.2.0 (Yoshua Wuyts)
- [[`b34449753b`](https://github.com/chooxide/webmanifest/commit/b34449753b6f212eef3b8e286498cbd43c287f9c)] background_color -> bg_color (Yoshua Wuyts)
- [[`9907a99f34`](https://github.com/chooxide/webmanifest/commit/9907a99f34ff3c75fbc88ab0c3b6df207f2f7df0)] Update changelog (Yoshua Wuyts)

### Stats
```diff
 CHANGELOG.md     | 32 ++++++++++++++++++++++++++++++++
 Cargo.toml       |  2 +-
 README.md        |  2 +-
 examples/main.rs |  2 +-
 src/lib.rs       |  4 ++--
 5 files changed, 37 insertions(+), 5 deletions(-)
```


## 2018-10-14, Version 0.1.2
### Commits
- [[`e6ba747a63`](https://github.com/chooxide/webmanifest/commit/e6ba747a6339a685361adb010fb4539876ae9496)] (cargo-release) version 0.1.2 (Yoshua Wuyts)
- [[`60b95b70ee`](https://github.com/chooxide/webmanifest/commit/60b95b70ee4de92d33803e2e00651d97121930e8)] upgrade versions (Yoshua Wuyts)
- [[`1780bca38e`](https://github.com/chooxide/webmanifest/commit/1780bca38eff8e5862ceb81b64404546d1a2e7cf)] fix extension detection (Yoshua Wuyts)
- [[`c3719b5cfb`](https://github.com/chooxide/webmanifest/commit/c3719b5cfbf357030ed2821ddaad6da7142fb055)] better docs (Yoshua Wuyts)
- [[`1b538aa7af`](https://github.com/chooxide/webmanifest/commit/1b538aa7afa106d55af02f7ea94ff7c108454f80)] readme example (Yoshua Wuyts)
- [[`37c36eb7c2`](https://github.com/chooxide/webmanifest/commit/37c36eb7c2e31f95fb0eb72a9322dfbfbab156d7)] more (Yoshua Wuyts)
- [[`22c835331b`](https://github.com/chooxide/webmanifest/commit/22c835331b57b1ca480eca62838fdfe9c5586f2d)] init (Yoshua Wuyts)

### Stats
```diff
 .github/CODE_OF_CONDUCT.md       |  75 ++++++++-
 .github/CONTRIBUTING.md          |  63 +++++++-
 .github/ISSUE_TEMPLATE.md        |  41 ++++-
 .github/PULL_REQUEST_TEMPLATE.md |  21 ++-
 .github/stale.yml                |  17 ++-
 .gitignore                       |   7 +-
 .travis.yml                      |  13 +-
 CERTIFICATE                      |  37 ++++-
 Cargo.toml                       |  16 ++-
 LICENSE-APACHE                   | 190 ++++++++++++++++++++-
 LICENSE-MIT                      |  21 ++-
 README.md                        |  65 +++++++-
 examples/main.rs                 |  17 ++-
 rustfmt.toml                     |   2 +-
 src/error.rs                     |  80 ++++++++-
 src/lib.rs                       | 378 ++++++++++++++++++++++++++++++++++++++++-
 16 files changed, 1043 insertions(+)
```


