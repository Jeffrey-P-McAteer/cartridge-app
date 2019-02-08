#![windows_subsystem = "windows"]

extern crate systray;

extern crate orbtk;

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



use orbtk::*;
struct MainView;
impl Widget for MainView {
    type Template = orbtk::widget::Template;
    fn create() -> Self::Template {
        Template::default()
    }
}
pub fn open_settings() {
  println!("Opening settings...");
  let mut application = Application::default();
  application
      .create_window()
      .bounds((100.0, 100.0, 800.0, 600.0))
      .title("Cartridge App Settings")
      .root(MainView::create())
      .build();
  application.run();
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



