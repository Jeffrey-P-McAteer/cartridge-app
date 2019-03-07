
use crate::*;

use std::time::Duration;
use std::thread;

use std::fs;
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
  handle_usbs_archlinux();
  // TODO
}

#[cfg(target_family = "unix")]
fn handle_usbs_archlinux() {
  match fs::read_dir("/run/media/") {
    Ok(paths) => {
      for path in paths {
        match path {
          Ok(path) => {
            let pstring = path.path().to_string_lossy().to_string();
            match fs::read_dir(pstring) {
              Ok(paths) => {
                for path in paths {
                  match path {
                    Ok(path) => {
                      let pstring = path.path().to_string_lossy().to_string();
                      check_pres(pstring);
                    }
                    Err(e) => {
                      println!("{}", e);
                    }
                  }
                }
              }
              Err(e) => {
                println!("{}", e);
              }
            }
          }
          Err(e) => {
            println!("{}", e);
          }
        }
      }
    }
    Err(e) => {
      println!("{}", e);
    }
  }
}

#[cfg(target_family = "windows")]
fn handle_usbs() {
  let paths = ["D:", "E:", "F:", "G:", "H:", "I:", "J:", "K:", "L:", "M:", "N:", "O:", "P:", "Q:", "R:", "S:", "T:", "U:", "V:" ];
  for p in paths.iter() {
    let path = Path::new(p);
    if path.exists() {
      println!("'{}' exists!", p);
      check_pres(p.to_string());
      
    }
  }
}

fn check_pres(usb_root: String) {
  #[cfg(target_family = "windows")]
  {
    let pres_p_s = format!("{}\\cartridge-pres.pdf", usb_root);
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
  #[cfg(target_family = "unix")]
  {
    let pres_p_s = format!("{}/cartridge-pres.pdf", usb_root);
    println!("pres_p_s={}", pres_p_s);
    let pres_p = Path::new(&pres_p_s);
    if pres_p.exists() {
      println!("Launching xpdf '{}'", pres_p_s);
      Command::new("xpdf") // TODO check if installed first
        .arg("-fullscreen")
        .arg(pres_p_s)
        .output()
        .expect("Failed to execute self");
      println!("Done with xpdf!");
    }
  }
}



