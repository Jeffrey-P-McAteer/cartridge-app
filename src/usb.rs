
use crate::*;

use std::time::Duration;
use std::thread;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

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
  let paths = ["D:", "E:", "F:", "G:", "H:", "I:", "J:", "K:", "L:", "M:", "N:", "O:", ];
  for p in paths.iter() {
    let path = Path::new(p);
    if path.exists() {
      println!("'{}' exists!", p);
      let pres_p_s = format!("{}\\cartridge-pres.pdf", p);
      let pres_p = Path::new(&pres_p_s);
      if pres_p.exists() {
        println!("Launching SumatraPDF '{}'", pres_p_s);
        Command::new("SumatraPDF.exe") // beause of os_main should be in local dir
          .arg("-presentation")
          .arg(pres_p_s)
          .output()
          .expect("Failed to execute self");
        println!("Done with SumatraPDF!");
      }
    }
  }
}

