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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
