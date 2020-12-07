#![forbid(unsafe_code, missing_debug_implementations, missing_docs)]
#![cfg_attr(test, deny(warnings))]

//! ## Example
//! ```rust
//! use webmanifest::{Manifest, Related};
//!
//! fn main() -> Result<(), failure::Error> {
//!   let name = "My Cool Application";
//!   let url = "https://play.google.com/store/apps/details?id=cheeaun.hackerweb";
//!   let manifest = Manifest::builder(name.into())
//!     .short_name("my app".into())
//!     .bg_color("#000".into())
//!     .related(&Related::new("play", url))
//!     .build()?;
//!   Ok(())
//! }
//! ```

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use failure::Error;

mod direction;
mod display_mode;
mod icon;
mod orientation;
mod related;

pub use crate::direction::Direction;
pub use crate::display_mode::DisplayMode;
pub use crate::icon::Icon;
pub use crate::orientation::Orientation;
pub use crate::related::Related;

/// The MIME type for `.webmanifest` files.
pub const MIME_TYPE_STR: &str = "application/manifest+json";

/// Create a new manifest builder.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest<'i, 'r> {
  name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  short_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  start_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "display")]
  display_mode: Option<DisplayMode>,
  #[serde(skip_serializing_if = "Option::is_none")]
  background_color: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  description: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "dir")]
  direction: Option<Direction>,
  #[serde(skip_serializing_if = "Option::is_none")]
  orientation: Option<Orientation>,
  #[serde(skip_serializing_if = "Option::is_none")]
  lang: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  scope: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  theme_color: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  prefer_related_applications: Option<bool>,
  #[serde(borrow)]
  icons: Vec<Icon<'i>>,
  #[serde(borrow)]
  related_applications: Vec<Related<'r>>,
}

impl<'i, 'r> Manifest<'i, 'r> {
  /// Create a new instance.
  ///
  /// ## Example
  /// ```rust
  /// # use webmanifest::Manifest;
  /// let name = "My Cool Application";
  /// let builder = Manifest::builder(name.into());
  /// ```
  #[must_use]
  #[inline]
  pub fn builder(name: String) -> Self {
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
  /// # use webmanifest::Manifest;
  /// # fn main() -> Result<(), failure::Error> {
  /// let name = "My Cool Application";
  /// let manifest = Manifest::builder(name.into()).build()?;
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
  /// # use webmanifest::Manifest;
  /// # fn main() -> Result<(), failure::Error> {
  /// let name = "My Cool Application";
  /// let manifest = Manifest::builder(name.into()).pretty()?;
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
  /// # use webmanifest::Manifest;
  /// # fn main() -> Result<(), failure::Error> {
  /// let name = "My Cool Application";
  /// let manifest = Manifest::builder(name.into())
  ///   .short_name("Cool App".into())
  ///   .build()?;
  /// # Ok(())}
  /// ```
  #[must_use]
  #[inline]
  pub fn short_name(mut self, name: String) -> Self {
    debug_assert!(name.len() <= 12);
    self.short_name = Some(name.into());
    self
  }

  /// Set the `start_url` value.
  ///
  /// ## Example
  /// ```rust
  /// # use webmanifest::Manifest;
  /// # fn main() -> Result<(), failure::Error> {
  /// let name = "My Cool Application";
  /// let manifest = Manifest::builder(name.into())
  ///   .start_url(".".into())
  ///   .build()?;
  /// # Ok(())}
  /// ```
  #[must_use]
  #[inline]
  pub fn start_url(mut self, url: String) -> Self {
    self.start_url = Some(url);
    self
  }

  /// Set the `display` value.
  ///
  /// ## Example
  /// ```rust
  /// # use webmanifest::{Manifest, DisplayMode};
  /// # fn main() -> Result<(), failure::Error> {
  /// let name = "My Cool Application";
  /// let manifest = Manifest::builder(name.into())
  ///   .display_mode(DisplayMode::Standalone)
  ///   .build()?;
  /// # Ok(())}
  /// ```
  #[must_use]
  #[inline]
  pub fn display_mode(mut self, mode: DisplayMode) -> Self {
    self.display_mode = Some(mode);
    self
  }

  /// Set the `background_color` value.
  ///
  /// ## Example
  /// ```rust
  /// # use webmanifest::Manifest;
  /// # fn main() -> Result<(), failure::Error> {
  /// let name = "My Cool Application";
  /// let manifest = Manifest::builder(name.into())
  ///   .bg_color("#000".into())
  ///   .build()?;
  /// # Ok(())}
  /// ```
  #[must_use]
  #[inline]
  pub fn bg_color(mut self, color: String) -> Self {
    self.background_color = Some(color);
    self
  }

  /// Set the `theme_color` value.
  ///
  /// ## Example
  /// ```rust
  /// # use webmanifest::Manifest;
  /// # fn main() -> Result<(), failure::Error> {
  /// let name = "My Cool Application";
  /// let manifest = Manifest::builder(name.into())
  ///   .theme_color("#000".into())
  ///   .build()?;
  /// # Ok(())}
  /// ```
  #[must_use]
  #[inline]
  pub fn theme_color(mut self, color: String) -> Self {
    self.theme_color = Some(color);
    self
  }

  /// Set the `description` value.
  ///
  /// ## Example
  /// ```rust
  /// # use webmanifest::Manifest;
  /// # fn main() -> Result<(), failure::Error> {
  /// let name = "My Cool Application";
  /// let desc = "It does many things.";
  /// let manifest = Manifest::builder(name.into())
  ///   .description(desc.into())
  ///   .build()?;
  /// # Ok(())}
  /// ```
  #[must_use]
  #[inline]
  pub fn description(mut self, desc: String) -> Self {
    self.description = Some(desc);
    self
  }

  /// Set the `lang` value.
  ///
  /// Specifies the primary language for the values in the name and short_name
  /// members. This value is a string containing a single language tag.
  ///
  /// ## Example
  /// ```rust
  /// # use webmanifest::Manifest;
  /// # fn main() -> Result<(), failure::Error> {
  /// let name = "My Cool Application";
  /// let lang = "en-US";
  /// let manifest = Manifest::builder(name.into())
  ///   .lang(lang.into())
  ///   .build()?;
  /// # Ok(())}
  /// ```
  #[must_use]
  #[inline]
  pub fn lang(mut self, lang: String) -> Self {
    self.lang = Some(lang);
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
  /// # use webmanifest::{Manifest, Orientation};
  /// # fn main() -> Result<(), failure::Error> {
  /// let name = "My Cool Application";
  /// let manifest = Manifest::builder(name.into())
  ///   .orientation(Orientation::Portrait)
  ///   .build()?;
  /// # Ok(())}
  /// ```
  #[must_use]
  #[inline]
  pub fn orientation(mut self, orientation: Orientation) -> Self {
    self.orientation = Some(orientation);
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
  /// # use webmanifest::{Manifest, Direction};
  /// # fn main() -> Result<(), failure::Error> {
  /// let name = "My Cool Application";
  /// let lang = "en-US";
  /// let manifest = Manifest::builder(name.into())
  ///   .direction(Direction::Ltr)
  ///   .build()?;
  /// # Ok(())}
  /// ```
  #[must_use]
  #[inline]
  pub fn direction(mut self, dir: Direction) -> Self {
    self.direction = Some(dir);
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
  /// # use webmanifest::{Manifest, Direction};
  /// # fn main() -> Result<(), failure::Error> {
  /// let name = "My Cool Application";
  /// let manifest = Manifest::builder(name.into())
  ///   .prefer_related_applications(true)
  ///   .build()?;
  /// # Ok(())}
  /// ```
  #[must_use]
  #[inline]
  pub fn prefer_related_applications(mut self, prefer: bool) -> Self {
    self.prefer_related_applications = Some(prefer);
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
  /// # use webmanifest::{Manifest, Direction};
  /// # fn main() -> Result<(), failure::Error> {
  /// let name = "My Cool Application";
  /// let manifest = Manifest::builder(name.into())
  ///   .scope("/myapp/".into())
  ///   .build()?;
  /// # Ok(())}
  /// ```
  #[must_use]
  #[inline]
  pub fn scope(mut self, scope: String) -> Self {
    self.scope = Some(scope);
    self
  }

  /// Add an `Icon` to the icons vector.
  ///
  /// ## Example
  /// ```rust
  /// # use webmanifest::{Manifest, Icon};
  /// # fn main() -> Result<(), failure::Error> {
  /// let name = "My Cool Application";
  /// let src = "images/touch/homescreen48.png";
  /// let manifest = Manifest::builder(name.into())
  ///   .icon(&Icon::new(&src, "48x48"))
  ///   .build()?;
  /// # Ok(())}
  /// ```
  #[must_use]
  #[inline]
  pub fn icon(mut self, icon: &'i Icon) -> Self {
    self.icons.push(icon.clone());
    self
  }

  /// Add an `Related` application to the `related_applications` vector.
  ///
  /// ## Example
  /// ```rust
  /// # use webmanifest::{Manifest, Related};
  /// # fn main() -> Result<(), failure::Error> {
  /// let name = "My Cool Application";
  /// let url = "https://play.google.com/store/apps/details?id=cheeaun.hackerweb";
  /// let manifest = Manifest::builder(name.into())
  ///   .related(&Related::new("play", url))
  ///   .build()?;
  /// # Ok(())}
  /// ```
  #[must_use]
  #[inline]
  pub fn related(mut self, related: &'r Related) -> Self {
    self.related_applications.push(related.clone());
    self
  }
}

// impl<'i, 'r> std::str::FromStr for &Manifest<'i, 'r> {
//   type Err = Error;

//   fn from_str(s: &str) -> Result<Self, Self::Err> {
//     let manifest: Manifest = serde_json::from_str(s)?;
//     Ok(&manifest)
//   }
// }
