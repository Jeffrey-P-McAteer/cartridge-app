
use crate::*;

use std::time::Duration;
use std::thread;

pub fn run_listener() {
  println!("Running USB Listener");
  loop {
    // Poll every 250 ms
    thread::sleep(Duration::from_millis(250));
    println!("Checking USB...");
    handle_usbs();
  }
}

#[cfg(target_family = "unix")]
fn handle_usbs() {
  
}

#[cfg(target_family = "windows")]
fn handle_usbs() {
  
}

