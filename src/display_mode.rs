/// Defines the developers’ preferred display mode for the website.
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
