#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]
#![forbid(unsafe_code, missing_debug_implementations)]

extern crate failure;
extern crate mime_guess;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use failure::Error;
use std::ffi::OsStr;
use std::path::Path;

/// Defines the developers’ preferred display mode for the website.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DisplayMode {
  /// All of the available display area is used and no user agent chrome is
  /// shown.
  #[serde(rename = "full-screen")]
  FullScreen,
  /// The application will look and feel like a standalone application. This can
  /// include the application having a different window, its own icon in the
  /// application launcher, etc. In this mode, the user agent will exclude UI
  /// elements for controlling navigation, but can include other UI elements
  /// such as a status bar.
  #[serde(rename = "standalone")]
  Standalone,
  /// The application will look and feel like a standalone application, but will
  /// have a minimal set of UI elements for controlling navigation. The elements
  /// will vary by browser.
  #[serde(rename = "minimal-ui")]
  MinimalUi,
  /// The application opens in a conventional browser tab or new window,
  /// depending on the browser and platform. This is the default.
  #[serde(rename = "browser")]
  Browser,
}

/// Add an icon to the web manifest.
///
/// ## Example Output
/// ```json
/// "icons": [{
///   "src": "images/touch/homescreen48.png",
///   "sizes": "48x48",
///   "type": "image/png"
/// }, {
///   "src": "images/touch/homescreen72.png",
///   "sizes": "72x72",
///   "type": "image/png"
/// }, {
///   "src": "images/touch/homescreen96.png",
///   "sizes": "96x96",
///   "type": "image/png"
/// }, {
///   "src": "images/touch/homescreen144.png",
///   "sizes": "144x144",
///   "type": "image/png"
/// }, {
///   "src": "images/touch/homescreen168.png",
///   "sizes": "168x168",
///   "type": "image/png"
/// }, {
///   "src": "images/touch/homescreen192.png",
///   "sizes": "192x192",
///   "type": "image/png"
/// }],
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Icon<'s> {
  src: &'s str,
  sizes: &'s str,
  #[serde(rename = "type")]
  icon_type: String,
}

impl<'s> Icon<'s> {
  /// Create a new `Icon` instance.
  ///
  /// ## Example
  /// ```rust
  /// # extern crate webmanifest;
  /// # use webmanifest::Icon;
  /// let src = "images/touch/homescreen48.png";
  /// let icon = Icon::new(&src, "48x48");
  /// ```
  #[must_use]
  #[inline]
  pub fn new(src: &'s str, sizes: &'s str) -> Self {
    let ext = Path::new(src)
      .extension()
      .and_then(OsStr::to_str)
      .unwrap_or("");
    let mime_type = mime_guess::get_mime_type(&ext);
    let icon_type = format!("{}/{}", mime_type.type_(), mime_type.subtype());
    Self {
      src,
      sizes,
      icon_type,
    }
  }
}

/// Create a new manifest builder.
#[derive(Debug, Clone, Serialize)]
pub struct Manifest<'s, 'i, 'r> {
  name: &'s str,
  short_name: Option<&'s str>,
  start_url: Option<&'s str>,
  #[serde(rename = "display")]
  display_mode: Option<DisplayMode>,
  background_color: Option<&'s str>,
  description: Option<&'s str>,
  icons: Vec<&'i Icon<'i>>,
  related_applications: Vec<&'r Related<'r>>,
}

impl<'s, 'i, 'r> Manifest<'s, 'i, 'r> {
  /// Create a new instance.
  ///
  /// ## Example
  /// ```rust
  /// # extern crate webmanifest;
  /// # use webmanifest::Manifest;
  /// let name = "My Cool Application";
  /// let builder = Manifest::builder(name);
  /// ```
  #[must_use]
  #[inline]
  pub fn builder(name: &'s str) -> Self {
    Self {
      name,
      short_name: None,
      description: None,
      start_url: None,
      display_mode: None,
      background_color: None,
      icons: vec![],
      related_applications: vec![],
    }
  }

  /// Finalize the builder and create the manifest.
  ///
  /// ## Example
  /// ```rust
  /// # extern crate webmanifest;
  /// # extern crate failure;
  /// # use webmanifest::Manifest;
  /// # fn main() -> Result<(), failure::Error> {
  /// let name = "My Cool Application";
  /// let manifest = Manifest::builder(name).build()?;
  /// # Ok(())}
  /// ```
  #[must_use]
  #[inline]
  pub fn build(self) -> Result<String, Error> {
    let manifest = serde_json::to_string(&self)?;
    Ok(manifest)
  }

  /// Finalize the builder and create a pretty representation of the manifest.
  ///
  /// ## Example
  /// ```rust
  /// # extern crate webmanifest;
  /// # extern crate failure;
  /// # use webmanifest::Manifest;
  /// # fn main() -> Result<(), failure::Error> {
  /// let name = "My Cool Application";
  /// let manifest = Manifest::builder(name).pretty()?;
  /// # Ok(())}
  /// ```
  #[must_use]
  #[inline]
  pub fn pretty(self) -> Result<String, Error> {
    let manifest = serde_json::to_string_pretty(&self)?;
    Ok(manifest)
  }

  /// Set the `short_name` value.
  ///
  /// ## Panics
  /// This will panic if the short name exceeds 12 characters.
  ///
  /// ## Example
  /// ```rust
  /// # extern crate webmanifest;
  /// # extern crate failure;
  /// # use webmanifest::Manifest;
  /// # fn main() -> Result<(), failure::Error> {
  /// let name = "My Cool Application";
  /// let manifest = Manifest::builder(name)
  ///   .short_name("Cool App")
  ///   .build()?;
  /// # Ok(())}
  /// ```
  #[must_use]
  #[inline]
  pub fn short_name(mut self, val: &'s str) -> Self {
    debug_assert!(val.len() < 12);
    self.short_name = Some(val);
    self
  }

  /// Set the `start_url` value.
  ///
  /// ## Example
  /// ```rust
  /// # extern crate webmanifest;
  /// # extern crate failure;
  /// # use webmanifest::Manifest;
  /// # fn main() -> Result<(), failure::Error> {
  /// let name = "My Cool Application";
  /// let manifest = Manifest::builder(name)
  ///   .start_url(".")
  ///   .build()?;
  /// # Ok(())}
  /// ```
  #[must_use]
  #[inline]
  pub fn start_url(mut self, val: &'s str) -> Self {
    self.start_url = Some(val);
    self
  }

  /// Set the `display` value.
  ///
  /// ## Example
  /// ```rust
  /// # extern crate webmanifest;
  /// # extern crate failure;
  /// # use webmanifest::{Manifest, DisplayMode};
  /// # fn main() -> Result<(), failure::Error> {
  /// let name = "My Cool Application";
  /// let manifest = Manifest::builder(name)
  ///   .display_mode(DisplayMode::Standalone)
  ///   .build()?;
  /// # Ok(())}
  /// ```
  #[must_use]
  #[inline]
  pub fn display_mode(mut self, val: DisplayMode) -> Self {
    self.display_mode = Some(val);
    self
  }

  /// Set the `background_color` value.
  ///
  /// ## Example
  /// ```rust
  /// # extern crate webmanifest;
  /// # extern crate failure;
  /// # use webmanifest::Manifest;
  /// # fn main() -> Result<(), failure::Error> {
  /// let name = "My Cool Application";
  /// let manifest = Manifest::builder(name)
  ///   .bg_color("#000")
  ///   .build()?;
  /// # Ok(())}
  /// ```
  #[must_use]
  #[inline]
  pub fn bg_color(mut self, val: &'s str) -> Self {
    self.background_color = Some(val);
    self
  }

  /// Set the `description` value.
  ///
  /// ## Example
  /// ```rust
  /// # extern crate webmanifest;
  /// # extern crate failure;
  /// # use webmanifest::Manifest;
  /// # fn main() -> Result<(), failure::Error> {
  /// let name = "My Cool Application";
  /// let desc = "It does many things.";
  /// let manifest = Manifest::builder(name)
  ///   .description(desc)
  ///   .build()?;
  /// # Ok(())}
  /// ```
  #[must_use]
  #[inline]
  pub fn description(mut self, val: &'s str) -> Self {
    self.description = Some(val);
    self
  }

  /// Add an `Icon` to the icons vector.
  ///
  /// ## Example
  /// ```rust
  /// # extern crate webmanifest;
  /// # extern crate failure;
  /// # use webmanifest::{Manifest, Icon};
  /// # fn main() -> Result<(), failure::Error> {
  /// let name = "My Cool Application";
  /// let src = "images/touch/homescreen48.png";
  /// let manifest = Manifest::builder(name)
  ///   .icon(&Icon::new(&src, "48x48"))
  ///   .build()?;
  /// # Ok(())}
  /// ```
  #[must_use]
  #[inline]
  pub fn icon(mut self, val: &'i Icon) -> Self {
    self.icons.push(val);
    self
  }

  /// Add an `Related` application to the `related_applications` vector.
  ///
  /// ## Example
  /// ```rust
  /// # extern crate webmanifest;
  /// # extern crate failure;
  /// # use webmanifest::{Manifest, Related};
  /// # fn main() -> Result<(), failure::Error> {
  /// let name = "My Cool Application";
  ///
  /// let url = "https://play.google.com/store/apps/details?id=cheeaun.hackerweb";
  /// let manifest = Manifest::builder(name)
  ///   .related(&Related::new("play", url))
  ///   .build()?;
  /// # Ok(())}
  /// ```
  #[must_use]
  #[inline]
  pub fn related(mut self, val: &'r Related) -> Self {
    self.related_applications.push(val);
    self
  }
}

/// An entry in an array of native applications that are installable by, or
/// accessible to, the underlying platform.
///
/// For example, a native Android application obtainable through the Google Play
/// Store. Such applications are intended to be alternatives to the website that
/// provides similar/equivalent functionality — like the native app version of
/// the website.
///
/// ## Example Output
/// ```json
/// "related_applications": [{
///   "platform": "play",
///   "url": "https://play.google.com/store/apps/details?id=cheeaun.hackerweb"
/// }]
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Related<'s> {
  platform: &'s str,
  url: &'s str,
}

impl<'s> Related<'s> {
  /// Create a new `Related` instance.
  ///
  /// ## Example
  /// ```rust
  /// # extern crate webmanifest;
  /// # use webmanifest::Related;
  /// let platform = "play";
  /// let url = "https://play.google.com/store/apps/details?id=cheeaun.hackerweb";
  /// let related = Related::new(platform, url);
  /// ```
  #[inline]
  #[must_use]
  pub fn new(platform: &'s str, url: &'s str) -> Self {
    Self { platform, url }
  }
}
