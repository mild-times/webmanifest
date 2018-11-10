extern crate failure;
extern crate serde_json;
extern crate webmanifest;

use webmanifest::Manifest;

use std::fs;

#[test]
fn deserialize() -> Result<(), failure::Error> {
  let location = "tests/fixtures/manifest-1.json";
  let content = fs::read_to_string(location)?;
  println!("{:?}", content);
  Ok(())
}
