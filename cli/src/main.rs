extern crate log;
extern crate simple_logger;

use log::Level;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  simple_logger::init_with_level(Level::Debug).unwrap();

  let user = tiktokrs::User::new(String::from("one_eared_uno"));
  let links : Vec<String> = user.get_all_video_page_links()?;

  for link in links.iter() {
    print!("{}", link);
  }

  Ok(())
}

