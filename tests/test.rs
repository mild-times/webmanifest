use webmanifest::Manifest;

use std::fs;

#[test]
fn deserialize() -> Result<(), failure::Error> {
  let location = "tests/fixtures/manifest-1.json";
  let content = fs::read_to_string(location)?;
  let _manifest: Manifest = serde_json::from_str(&content)?;

  // assert_eq!(manifest.name(), Some("hackerweb".to_string()));
  // assert_eq!(manifest.short_name(), "HackerWeb");
  // assert_eq!(manifest.start_url(), ".");
  // assert_eq!(manifest.display(), webmanifest::DisplayMode::Standalone);
  // assert_eq!(manifest.background_color(), "#fff");
  // assert_eq!(manifest.description(), "A simply readable Hacker News app.".to_string());
  Ok(())
}
