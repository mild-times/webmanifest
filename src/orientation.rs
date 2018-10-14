/// Defines the default orientation for all the website's top level browsing
/// contexts.
/// ## Example
/// ```rust
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Orientation {
  #[serde(rename = "any")]
  Any,
  #[serde(rename = "natural")]
  Natural,
  #[serde(rename = "landscape")]
  Landscape,
  #[serde(rename = "landscape-primary")]
  LandscapPrimary,
  #[serde(rename = "landscape-secondary")]
  LandscapSecondary,
  #[serde(rename = "portrait")]
  Portrait,
  #[serde(rename = "portrait-primary")]
  PortraitPrimary,
  #[serde(rename = "portrait-secondary")]
  PortraitSecondary,
}
