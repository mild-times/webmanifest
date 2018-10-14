/// Defines the default orientation for all the website's top level browsing
/// contexts.
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Orientation {
  ///
  #[serde(rename = "any")]
  Any,
  ///
  #[serde(rename = "natural")]
  Natural,
  ///
  #[serde(rename = "landscape")]
  Landscape,
  ///
  #[serde(rename = "landscape-primary")]
  LandscapPrimary,
  ///
  #[serde(rename = "landscape-secondary")]
  LandscapSecondary,
  ///
  #[serde(rename = "portrait")]
  Portrait,
  ///
  #[serde(rename = "portrait-primary")]
  PortraitPrimary,
  ///
  #[serde(rename = "portrait-secondary")]
  PortraitSecondary,
}
