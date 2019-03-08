
use crate::*;

use std::time::Duration;
use std::thread;

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::{Command,Child};

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
                      check_pres(pstring.clone());
                      check_vid(pstring.clone());
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
      check_vid(p.to_string());
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
      let mut child = Command::new("SumatraPDF.exe") // beause of os_main should be in local dir
        .arg("-page")
        .arg("1")
        .arg("-presentation")
        .arg(pres_p_s.clone())
        .spawn()
        .expect("Failed to execute xpdf");
      
      // Loops
      kill_child_when_file_moves(&mut child, pres_p, "SumatraPDF");
      
    }
  }
  #[cfg(target_family = "unix")]
  {
    let pres_p_s = format!("{}/cartridge-pres.pdf", usb_root);
    println!("pres_p_s={}", pres_p_s);
    let pres_p = Path::new(&pres_p_s);
    if pres_p.exists() {
      println!("Launching xpdf '{}'", pres_p_s);
      let mut child = Command::new("xpdf") // TODO check if installed first
        .arg("-fullscreen")
        .arg(pres_p_s.clone())
        .spawn()
        .expect("Failed to execute xpdf");
      
      // Loops
      kill_child_when_file_moves(&mut child, pres_p, "xpdf");
      
    }
  }
}


fn check_vid(usb_root: String) {
  for end in ["mp4", "mov", "avi"].iter() {
    #[cfg(target_family = "windows")]
    {
      let vid_p_s = format!("{}\\cartridge-vid.{}", usb_root, end);
      let vid_p = Path::new(&vid_p_s);
      if vid_p.exists() {
        println!("Launching mpv '{}'", vid_p_s);
        let mut child = Command::new("mpv.exe") // beause of os_main should be in local dir
          .arg("--fs")
          .arg(vid_p_s.clone())
          .spawn()
          .expect("Failed to execute mpv");
          
        // Loops
        kill_child_when_file_moves(&mut child, vid_p, "mpv");
        
        println!("Done with mpv!");
      }
    }
    #[cfg(target_family = "unix")]
    {
      let vid_p_s = format!("{}/cartridge-vid.{}", usb_root, end);
      println!("vid_p_s={}", vid_p_s);
      let vid_p = Path::new(&vid_p_s);
      if vid_p.exists() {
        println!("Launching mpv '{}'", vid_p_s);
        let mut child = Command::new("mpv") // TODO check if installed first
          .arg("--fs")
          .arg(vid_p_s.clone())
          .spawn()
          .expect("Failed to execute mpv");
        
        kill_child_when_file_moves(&mut child, vid_p, "mpv");
        
        println!("Done with mpv!");
      }
    }
  }
}

fn kill_child_when_file_moves(child: &mut Child, file_p: &Path, child_name: &str) {
  // Loop; exit if the child dies, kill child if USB removed/file no longer exists
  let mut loop_i = 0;
  loop {
    if (loop_i % 6) == 0 { // force a focus on the child using PID every 1.5 seconds or so
      bring_child_to_foreground(child_name);
      loop_i = 0;
    }
    loop_i += 1;
    thread::sleep(Duration::from_millis(250));
    
    match child.try_wait() {
      Ok(Some(status)) => {
        println!("Done with xpdf! (status={})", status);
        break;
      }
      Ok(None) => {
        println!("Child still running...");
      }
      Err(e) => {
        println!("error attempting to wait: {}", e)
      }
    }
    // Child is still running, let's see if the file still exists
    if ! file_p.exists() {
      // File removed, kill child
      match child.kill() {
        Ok(_) => {
          println!("Child killed because USB removed");
        }
        Err(e) => {
          println!("Child cannot be killed: {}", e);
        }
      }
      break;
    }
  }
}

fn bring_child_to_foreground(child_name: &str) {
  #[cfg(target_family = "windows")]
  {
    //let payload = format!("(New-Object -ComObject WScript.Shell).AppActivate((Get-Process {}).MainWindowTitle)", child_name);
    // Wheeeee
    // let payload = format!(
    //   "Add-Type 'using System; using System.Runtime.InteropServices; public class Tricks {{ [DllImport(\"user32.dll\")] [return: MarshalAs(UnmanagedType.Bool)] public static extern bool SetForegroundWindow(IntPtr hWnd); }} ' ;  $h = (Get-Process {} ).MainWindowHandle ; [void] [Tricks]::SetForegroundWindow($h)",
    //   child_name
    // );
    // Command::new("PSRun.exe") // This is extracted at startup
    //   .arg(&payload)
    //   .output()
    //   .expect("Failed to execute PSRun.exe");
    
    println!("Turns out there's no good way");
    
  }
  #[cfg(target_family = "unix")]
  {
    println!("bring_child_to_foreground unimplemented in unix (child_name={})", child_name);
  }
}

