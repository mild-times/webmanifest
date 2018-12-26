/// Defines the default orientation for all the website's top level browsing
/// contexts. [Read more.](https://www.w3.org/TR/screen-orientation)
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Orientation {
  /// Enable `portrait-primary`, `portrait-secondary`, `landscape-primary` and
  /// `landscape-secondary` orientations.
  #[serde(rename = "any")]
  Any,
  /// Enable `portrait-primary` or `landscape-primary` orientations such as the
  /// associated current orientation angle is 0.
  #[serde(rename = "natural")]
  Natural,
  /// The screen width is greater than the screen height. This enables both
  /// `landscape-primary` and `landscape-secondary` orientations.
  #[serde(rename = "landscape")]
  Landscape,
  /// The screen width is greater than the screen height, and there are multiple
  /// orientations possible. This is the first of those possible orientations.
  #[serde(rename = "landscape-primary")]
  LandscapePrimary,
  /// The screen width is greater than the screen height, and there are multiple
  /// orientations possible. This is the second of those possible orientations.
  #[serde(rename = "landscape-secondary")]
  LandscapeSecondary,
  /// The screen height is greater than the screen width. This enables both
  /// `portrait-primary` and `portrait-secondary` orientations.
  #[serde(rename = "portrait")]
  Portrait,
  /// The screen height is greater than the screen width, and there are multiple
  /// orientations possible. This is the first of those possible orientations.
  #[serde(rename = "portrait-primary")]
  PortraitPrimary,
  /// The screen height is greater than the screen width, and there are multiple
  /// orientations possible. This is the second of those possible orientations.
  #[serde(rename = "portrait-secondary")]
  PortraitSecondary,
}
