use mime_guess;

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
    let icon_type = mime_guess::guess_mime_type(&src).to_string();
    Self {
      src,
      sizes,
      icon_type,
    }
  }
}
