
#[cfg(target_os = "windows")]
extern crate winapi;

#[cfg(target_os = "windows")]
mod win_tray;

#[cfg(target_os = "linux")]
mod linux_tray;

fn main() {
  make_tray();
}

#[cfg(target_os = "windows")]
fn make_tray() {
  win_tray::run();
}

#[cfg(target_os = "linux")]
fn make_tray() {
  linux_tray::run();
}
