
extern crate systray;

//#[cfg(target_os = "windows")]
//extern crate winapi;

//#[cfg(target_os = "windows")]
//mod win_tray;

//#[cfg(target_os = "linux")]
//mod linux_tray;

fn main() {
  make_tray();
}

fn make_tray() {
  let mut app;
  match systray::Application::new() {
      Ok(w) => app = w,
      Err(_) => panic!("Can't create window!")
  }
  // w.set_icon_from_file(&"C:\\Users\\qdot\\code\\git-projects\\systray-rs\\resources\\rust.ico".to_string());
  // w.set_tooltip(&"Whatever".to_string());
  app.set_icon_from_file(&"/usr/share/gxkb/flags/ua.png".to_string()).ok();
  app.add_menu_item(&"Print a thing".to_string(), |_| {
      println!("Printing a thing!");
  }).ok();
  app.add_menu_item(&"Add Menu Item".to_string(), |window| {
      window.add_menu_item(&"Interior item".to_string(), |_| {
          println!("what");
      }).ok();
      window.add_menu_separator().ok();
  }).ok();
  app.add_menu_separator().ok();
  app.add_menu_item(&"Quit".to_string(), |window| {
      window.quit();
  }).ok();
  println!("Waiting on message!");
  app.wait_for_message();
}

// #[cfg(target_os = "windows")]
// fn make_tray() {
//   win_tray::run();
// }

// #[cfg(target_os = "linux")]
// fn make_tray() {
//   linux_tray::run();
// }
