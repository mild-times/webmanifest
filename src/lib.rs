#![forbid(unsafe_code, missing_debug_implementations, missing_docs)]
#![cfg_attr(test, deny(warnings))]

//! ## Example
//! ```rust
//! extern crate webmanifest;
//! extern crate failure;
//!
//! use webmanifest::{Manifest, Related};
//!
//! fn main() -> Result<(), failure::Error> {
//!   let name = "My Cool Application";
//!   let url = "https://play.google.com/store/apps/details?id=cheeaun.hackerweb";
//!   let manifest = Manifest::builder(name)
//!     .short_name("my app")
//!     .bg_color("#000")
//!     .related(&Related::new("play", url))
//!     .build()?;
//!   Ok(())
//! }
//! ```

extern crate failure;
extern crate mime_guess;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use failure::Error;

mod orientation;
mod related;
mod display_mode;
mod direction;
mod icon;

pub use orientation::Orientation;
pub use direction::Direction;
pub use display_mode::DisplayMode;
pub use related::Related;
pub use icon::Icon;

/// The MIME type for `.webmanifest` files.
pub const MIME_TYPE_STR: &'static str = "application/manifest+json";

/// Create a new manifest builder.
#[derive(Debug, Clone, Serialize)]
pub struct Manifest<'s, 'i, 'r> {
  name: &'s str,
  #[serde(skip_serializing_if = "Option::is_none")]
  short_name: Option<&'s str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  start_url: Option<&'s str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "display")]
  display_mode: Option<DisplayMode>,
  #[serde(skip_serializing_if = "Option::is_none")]
  background_color: Option<&'s str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  description: Option<&'s str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "dir")]
  direction: Option<Direction>,
  #[serde(skip_serializing_if = "Option::is_none")]
  orientation: Option<Orientation>,
  #[serde(skip_serializing_if = "Option::is_none")]
  lang: Option<&'s str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  scope: Option<&'s str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  theme_color: Option<&'s str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  prefer_related_applications: Option<bool>,
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
      orientation: None,
      direction: None,
      lang: None,
      background_color: None,
      theme_color: None,
      scope: None,
      prefer_related_applications: None,
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
    debug_assert!(val.len() <= 12);
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

  /// Set the `theme_color` value.
  ///
  /// ## Example
  /// ```rust
  /// # extern crate webmanifest;
  /// # extern crate failure;
  /// # use webmanifest::Manifest;
  /// # fn main() -> Result<(), failure::Error> {
  /// let name = "My Cool Application";
  /// let manifest = Manifest::builder(name)
  ///   .theme_color("#000")
  ///   .build()?;
  /// # Ok(())}
  /// ```
  #[must_use]
  #[inline]
  pub fn theme_color(mut self, val: &'s str) -> Self {
    self.theme_color = Some(val);
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

  /// Set the `lang` value.
  ///
  /// Specifies the primary language for the values in the name and short_name
  /// members. This value is a string containing a single language tag.
  ///
  /// ## Example
  /// ```rust
  /// # extern crate webmanifest;
  /// # extern crate failure;
  /// # use webmanifest::Manifest;
  /// # fn main() -> Result<(), failure::Error> {
  /// let name = "My Cool Application";
  /// let lang = "en-US";
  /// let manifest = Manifest::builder(name)
  ///   .lang(lang)
  ///   .build()?;
  /// # Ok(())}
  /// ```
  #[must_use]
  #[inline]
  pub fn lang(mut self, val: &'s str) -> Self {
    self.lang = Some(val);
    self
  }

  /// Set the `orientation` value.
  ///
  /// Specifies a boolean value that hints for the user agent to indicate to the
  /// user that the specified native applications are recommended over the
  /// website. This should only be used if the related native apps really do
  /// offer something that the website can't.
  ///
  /// ## Example
  /// ```rust
  /// # extern crate webmanifest;
  /// # extern crate failure;
  /// # use webmanifest::{Manifest, Orientation};
  /// # fn main() -> Result<(), failure::Error> {
  /// let name = "My Cool Application";
  /// let manifest = Manifest::builder(name)
  ///   .orientation(Orientation::Portrait)
  ///   .build()?;
  /// # Ok(())}
  /// ```
  #[must_use]
  #[inline]
  pub fn orientation(mut self, val: Orientation) -> Self {
    self.orientation = Some(val);
    self
  }

  /// Set the `dir` value.
  ///
  /// Specifies the primary text direction for the name, short_name, and
  /// description members. Together with the lang member, it helps the correct
  /// display of right-to-left languages.
  ///
  /// ## Example
  /// ```rust
  /// # extern crate webmanifest;
  /// # extern crate failure;
  /// # use webmanifest::{Manifest, Direction};
  /// # fn main() -> Result<(), failure::Error> {
  /// let name = "My Cool Application";
  /// let lang = "en-US";
  /// let manifest = Manifest::builder(name)
  ///   .direction(Direction::Ltr)
  ///   .build()?;
  /// # Ok(())}
  /// ```
  #[must_use]
  #[inline]
  pub fn direction(mut self, val: Direction) -> Self {
    self.direction = Some(val);
    self
  }

  /// Set the `prefer_related_applications` value.
  ///
  /// Specifies a boolean value that hints for the user agent to indicate to the
  /// user that the specified native applications are recommended over the
  /// website. This should only be used if the related native apps really do
  /// offer something that the website can't.
  ///
  /// ## Note
  /// If omitted, the value defaults to `false`.
  ///
  /// ## Example
  /// ```rust
  /// # extern crate webmanifest;
  /// # extern crate failure;
  /// # use webmanifest::{Manifest, Direction};
  /// # fn main() -> Result<(), failure::Error> {
  /// let name = "My Cool Application";
  /// let manifest = Manifest::builder(name)
  ///   .prefer_related_applications(true)
  ///   .build()?;
  /// # Ok(())}
  /// ```
  #[must_use]
  #[inline]
  pub fn prefer_related_applications(mut self, val: bool) -> Self {
    self.prefer_related_applications = Some(val);
    self
  }

  /// Set the `scope` value.
  ///
  /// Defines the navigation scope of this website's context. This restricts
  /// what web pages can be viewed while the manifest is applied. If the user
  /// navigates outside the scope, it returns to a normal web page inside a
  /// browser tab/window.
  ///
  /// If the scope is a relative URL, the base URL will be the URL of the
  /// manifest.
  ///
  /// ## Example
  /// ```rust
  /// # extern crate webmanifest;
  /// # extern crate failure;
  /// # use webmanifest::{Manifest, Direction};
  /// # fn main() -> Result<(), failure::Error> {
  /// let name = "My Cool Application";
  /// let manifest = Manifest::builder(name)
  ///   .scope("/myapp/")
  ///   .build()?;
  /// # Ok(())}
  /// ```
  #[must_use]
  #[inline]
  pub fn scope(mut self, val: &'s str) -> Self {
    self.scope = Some(val);
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
