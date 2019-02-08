
use std::fs::File;
use std::io::prelude::*;

pub fn os_main() {
  // Take embedded SDL2.dll and put it in current dir
  {
    let mut file = File::create("SDL2.dll").expect("Could not create SDL2.dll");
    // Write a slice of bytes to the file
    match file.write_all(include_bytes!("SDL2.dll")) {
      Ok(_) => { }
      Err(e) => {
        println!("{}", e);
      }
    }
  }
  
  // Windows doesn't exactly have a stable temp file API
  // and I'm not going to invent one, we'll just dump the icon wherever we currently are.
  {
    let mut file = File::create("icon.png").expect("Could not create icon.png");
    // Write a slice of bytes to the file
    match file.write_all(include_bytes!("icon.png")) {
      Ok(_) => { }
      Err(e) => {
        println!("{}", e);
      }
    }
  }
  crate::make_tray( "icon.png".to_string() );
}

