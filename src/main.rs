
extern crate systray;

#[cfg(target_family = "unix")]
extern crate mktemp;

#[cfg(target_family = "unix")]
mod nixmain;

#[cfg(target_family = "windows")]
mod winmain;

fn main() {
  #[cfg(target_family = "unix")]
  nixmain::os_main();
  #[cfg(target_family = "windows")]
  winmain::os_main();
}

pub fn open_settings() {
  println!("Opening settings...");
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



