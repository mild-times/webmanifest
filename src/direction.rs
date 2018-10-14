/// Specifies the primary text direction for the `name`, `short_name`, and
/// `description` members.
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Direction {
  /// left-to-right
  #[serde(rename = "ltr")]
  Ltr,
  /// right-to-left
  #[serde(rename = "rtl")]
  Rtl,
  /// Hints to the browser to use the [Unicode bidirectional
  /// algorithm](https://developer.mozilla.org/en-US/docs/Web/Localization/Unicode_Bidirectional_Text_Algorithm)
  /// to make a best guess about the text's direction.
  #[serde(rename = "auto")]
  Auto,
}
