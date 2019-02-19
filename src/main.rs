#![windows_subsystem = "windows"]

extern crate systray;

#[cfg(target_family = "unix")]
extern crate mktemp;

extern crate piston_window;

use piston_window::*;

use std::env;
use std::io::Write;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
  if let Some(arg1) = env::args().nth(1) {
    if arg1 == "settings" {
      open_settings();
      return;
    }
  }
  os_main();
}

#[cfg(target_family = "unix")]
fn os_main() {
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
#[cfg(target_family = "unix")]
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


#[cfg(target_family = "windows")]
fn os_main() {
  // Windows doesn't exactly have a stable temp file API
  // and I'm not going to invent one, we'll just dump the icon wherever we currently are.
  if ! Path::new("icon.png").exists() {
    let mut file = File::create("icon.png").expect("Could not create icon.png");
    // Write a slice of bytes to the file
    match file.write_all(include_bytes!("../icon.png")) {
      Ok(_) => { }
      Err(e) => {
        println!("{}", e);
      }
    }
  }
  make_tray( "icon.png".to_string() );
}

pub fn open_settings() {
  println!("Opening settings...");
  let mut window: PistonWindow = PistonWindow::new(
        OpenGL::V3_3,
        0,
        WindowSettings::new("Cartridge App Settings", [640, 480])
            .opengl(OpenGL::V3_3)
            //.srgb(false)
            .build()
            .unwrap(),
    );
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0],
                      c.transform, g);
        });
    }
}


pub fn make_tray(icon_path: String) {
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
    #[cfg(target_family = "windows")]
    app.set_icon_from_resource(&icon_path).ok();
  }
  app.add_menu_item(&"Cartridge App".to_string(), |_| {
    
  }).ok();
  app.add_menu_separator().ok();
  app.add_menu_item(&"Open Settings".to_string(), |_window| {
      crate::open_settings();
  }).ok();
  app.add_menu_item(&"Quit".to_string(), |window| {
      window.quit();
  }).ok();
  println!("Beginning event loop...");
  app.wait_for_message();
}
