
extern crate systray;
extern crate mktemp;

use std::io::prelude::*;

fn main() {
  let icon_tmp_f = extract_icon();
  match icon_tmp_f {
    Some(icon_tmp_f) => {
      make_tray( format!("{}", icon_tmp_f.path()) );
    }
    None => {
      make_tray( "".to_string() );
    }
  }
  
}

// returns full path to icon
fn extract_icon() -> Option<mktemp::TempFile> {
  let icon_bytes = include_bytes!("../icon.png");
  match mktemp::TempFile::new("icon", ".png") {
    Ok(mut temp_file) => {
      match temp_file.inner().write_all(icon_bytes) {
        Ok(_) => { }
        Err(e) => {
          println!("{}", e);
          return None;
        }
      }
      return Some(temp_file);
    }
    Err(e) => {
      println!("{}", e);
      return None;
    }
  }
}

fn make_tray(icon_path: String) {
  println!("icon_path={}", icon_path);
  let mut app;
  match systray::Application::new() {
      Ok(w) => app = w,
      Err(_) => panic!("Can't create window!")
  }
  // w.set_icon_from_file(&"C:\\Users\\qdot\\code\\git-projects\\systray-rs\\resources\\rust.ico".to_string());
  // w.set_tooltip(&"Whatever".to_string());
  if icon_path.len() > 1 {
    app.set_icon_from_file(&icon_path).ok();
  }
  app.add_menu_item(&"Cartridge App".to_string(), |_| {
    
  }).ok();
  app.add_menu_separator().ok();
  app.add_menu_item(&"Open Settings".to_string(), |_window| {
      open_settings();
  }).ok();
  app.add_menu_item(&"Quit".to_string(), |window| {
      window.quit();
  }).ok();
  println!("Beginning event loop...");
  app.wait_for_message();
}

fn open_settings() {
  println!("Opening settings...");
}

