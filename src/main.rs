#![windows_subsystem = "windows"]

#[macro_use] extern crate conrod;
// https://github.com/DarpGosaNiled/conrod_hello_world/blob/master/src/main.rs

use conrod::{Positionable, Colorable, Widget};
use conrod::backend::glium::glium::{self, Surface};

extern crate systray;

#[cfg(target_family = "unix")]
extern crate mktemp;

#[cfg(target_family = "unix")]
extern crate dbus;

extern crate winapi;

use std::env;
use std::io::Write;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::thread;

mod shims;
mod gui;
mod usb;

fn main() {
  if let Some(arg1) = env::args().nth(1) {
    if arg1 == "settings" {
      gui::draw_settings_win();
      return;
    }
  }
  // Spawn thread to observe USB changes
  thread::spawn(move || {
    usb::run_listener();
  });
  os_main();
}

#[cfg(target_family = "unix")]
fn os_main() {
  let icon_tmp_f = extract_icon();
  match icon_tmp_f {
    Some(icon_tmp_f) => {
      gui::make_tray( format!("{}", icon_tmp_f.path()) );
    }
    None => {
      gui::make_tray( "".to_string() );
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
  if ! Path::new("icon.ico").exists() {
    let mut file = File::create("icon.ico").expect("Could not create icon.ico");
    // Write a slice of bytes to the file
    match file.write_all(include_bytes!("../icon.ico")) {
      Ok(_) => { }
      Err(e) => {
        println!("{}", e);
      }
    }
  }
  // Do the same for SumatraPDF.exe
  if ! Path::new("SumatraPDF.exe").exists() {
    let mut file = File::create("SumatraPDF.exe").expect("Could not create SumatraPDF.exe");
    // Write a slice of bytes to the file
    match file.write_all(include_bytes!("../assets/SumatraPDF.exe")) {
      Ok(_) => { }
      Err(e) => {
        println!("{}", e);
      }
    }
  }
  // Do the same for mpv.exe
  if ! Path::new("mpv.exe").exists() {
    let mut file = File::create("mpv.exe").expect("Could not create mpv.exe");
    // Write a slice of bytes to the file
    match file.write_all(include_bytes!("../assets/mpv.exe")) {
      Ok(_) => { }
      Err(e) => {
        println!("{}", e);
      }
    }
  }
  // Hey, guess what else?
  // if ! Path::new("PSRun.exe").exists() {
  //   let mut file = File::create("PSRun.exe").expect("Could not create PSRun.exe");
  //   // Write a slice of bytes to the file
  //   match file.write_all(include_bytes!("../assets/PSRun.exe")) {
  //     Ok(_) => { }
  //     Err(e) => {
  //       println!("{}", e);
  //     }
  //   }
  // }
  match env::current_dir() {
    Ok(dir_as_buff) => {
      let dir_as_s = dir_as_buff.as_path().to_string_lossy().to_string();
      let abs_icon_path = format!("{}\\icon.ico", dir_as_s);
      gui::make_tray(abs_icon_path);
    }
    Err(e) => {
      println!("{}", e);
      gui::make_tray( "icon.ico".to_string() );
    }
  }
}

pub fn open_settings() {
  use std::process::Command;
  println!("Opening settings...");
  thread::spawn(move || {
    let self_exe_path = shims::get_path_to_self_exe();
    println!("self_exe_path={}", self_exe_path);
    Command::new(self_exe_path)
        .arg("settings")
        .output()
        .expect("Failed to execute self");
    println!("Done with GUI!");

    // TODO read in settings from ~/.c
    
  });
}
