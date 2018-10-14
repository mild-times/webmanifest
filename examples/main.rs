extern crate failure;
extern crate webmanifest;

use webmanifest::{Icon, Manifest, Related};

fn main() -> Result<(), failure::Error> {
  let name = "My Cool Application";
  let url = "https://play.google.com/store/apps/details?id=cheeaun.hackerweb";
  let manifest = Manifest::builder(name)
    .short_name("my app")
    .bg_color("#000")
    .related(&Related::new("play", url))
    .icon(&Icon::new("/icon.png", "48x48"))
    .pretty()?;
  println!("{}", manifest);
  Ok(())
}
