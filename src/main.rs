#![windows_subsystem = "windows"]

extern crate systray;

extern crate orbtk;

#[cfg(target_family = "unix")]
extern crate mktemp;

use std::env;

#[cfg(target_family = "unix")]
mod nixmain;

#[cfg(target_family = "windows")]
mod winmain;

fn main() {
  if let Some(arg1) = env::args().nth(1) {
    if arg1 == "settings" {
      open_settings();
      return;
    }
  }
  // Launch System Tray
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
          .parent_type(orbtk::ParentType::Multi) // we hold multiple children
          .child(
            Grid::create()
                    .columns(
                        Columns::create()
                            .column("*")
                            .column("Auto") // Column::create().width(ColumnWidth::Auto).build()
                            .column(50.0)
                            .build(),
                    )
                    .rows(
                        Rows::create()
                          .row(Row::create().height( RowHeight::Height(56.0) ).build()) 
                          .row("*")
                          .build()
                    )
                    .child(
                        Grid::create()
                            .selector("navbar")
                            .margin((0.0, 0.0, 0.0, 5.0))
                            .attach_property(GridColumn(0))
                            .attach_property(GridRow(0))
                            .child(
                                TextBlock::create()
                                    .text("Cartridge App Settings")
                                    .horizontal_alignment("Center")
                                    .vertical_alignment("Center"),
                            ),
                    )
                    .child(
                        Grid::create()
                            .selector("goldendream")
                            .attach_property(GridColumn(0))
                            .attach_property(GridRow(1))
                            .attach_property(ColumnSpan(3))
                            .child(
                                TextBlock::create()
                                    .text("(0,1) - ColumnSpan 3")
                                    .selector("goldendream")
                                    .horizontal_alignment(HorizontalAlignment(Alignment::Center))
                                    .vertical_alignment(VerticalAlignment(Alignment::Center)),
                            ),
                    ),
          )
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
      .theme(
        Theme::create()
          .extension_css(r#"
* {
    font-size: 56;
}

lynch {
    background: #647b91;
}

bluebayoux {
    background: #516475;
}

linkwater {
    background: #dfebf5;
    color: #3b434a;
}

goldendream {
    background: #efd035;
    color: #3b434a;
}

"#)
          .build(),
      )
      //.resizable(true) // when resize==false forces floating in i3
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
