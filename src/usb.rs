
use crate::*;

use std::time::Duration;
use std::thread;

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::{Command,Child};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn run_listener() {
  println!("Running USB Listener");
  //let was_able_to_use_dbus = handle_usbs_dbus();
  //if ! was_able_to_use_dbus {
    loop {
      // Poll every 450 ms
      thread::sleep(Duration::from_millis(450));
      println!("Checking USB...");
      handle_usbs();
    }
  //}
}

#[cfg(target_family = "unix")]
fn handle_usbs() {
  handle_usbs_archlinux();
  // TODO
}

// This is more efficient than continuously polling
// the expected mount path, but not all OSes will be
// running DBUS
#[cfg(target_family = "unix")]
fn handle_usbs_dbus() -> bool {
  match dbus::Connection::get_private(dbus::BusType::Session) {
    Ok(c) => {
      
      // See https://dbus.freedesktop.org/doc/dbus-specification.html#message-bus-routing-match-rules
      match c.add_match("type='signal',interface='org.gtk.Private.RemoteVolumeMonitor'") {
        Ok(_) => {}
        Err(e) => {
          // Interface invalid or something
          println!("{}", e);
          return false;
        }
      }
      
      // TODO other matches for other OSes?
      
      // Pretent this is infinite
      let mut last_handled_time = SystemTime::now();
      for _conn_item in c.iter(i32::max_value()) {
        //println!("conn_item={:?}", conn_item);
        // Sleep 25ms, Check USB
        thread::sleep(Duration::from_millis(25));
        
        let duration = SystemTime::now().duration_since(last_handled_time).expect("Time travel accidentially enabled");
        // Only perform action if events are more than 2 seconds apart
        if duration.as_secs() > 2 {
          // O(1), hits all USBs
          last_handled_time = SystemTime::now();
          thread::spawn(move || {
            handle_usbs();
          });
        }
      }
      return true;
    }
    Err(e) => {
      println!("{}", e);
    }
  }
  return false;
}

// Windows has no dbus; this is easier than refactoring run_listener
#[cfg(target_family = "windows")]
fn handle_usbs_dbus() -> bool {
  return false;
}

#[cfg(target_family = "unix")]
fn handle_usbs_archlinux() {
  handle_usb_mount_dir_2("/run/media/");
  handle_usb_mount_dir_1("/mnt/");
}

#[cfg(target_family = "unix")]
fn handle_usb_mount_dir_2(directory: &str) {
  match fs::read_dir(directory) {
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

#[cfg(target_family = "unix")]
fn handle_usb_mount_dir_1(directory: &str) {
  match fs::read_dir(directory) {
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
    { // PDF
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
    { // PPTX
      let pres_p_s = format!("{}\\cartridge-pres.pptx", usb_root);
      let pres_p = Path::new(&pres_p_s);
      if pres_p.exists() {
        // Copy it to a PPS file
        let pres_pps = ".\\cartridge-pres.pps".to_string();
        match fs::copy(pres_p_s.clone(), pres_pps.clone()) {
          Ok(_) => {
            println!("Launching default app for file '{}'", pres_pps.clone());
            let mut child = Command::new("cmd.exe")
              .arg("/C")
              .arg(pres_pps.clone())
              .spawn()
              .expect("Failed to execute default app");
            
            // Sleep for 2.5 seconds first
            thread::sleep(Duration::from_millis(2500));
            
            // Loops
            kill_child_when_file_moves(&mut child, pres_p, "POWERPNT.EXE");
          }
          Err(e) => {
            println!("{}", e);
          }
        }
      }
    }
    { // PPT
      let pres_p_s = format!("{}\\cartridge-pres.ppt", usb_root);
      let pres_p = Path::new(&pres_p_s);
      if pres_p.exists() {
        // Copy it to a PPS file
        let pres_pps = ".\\cartridge-pres.pps".to_string();
        match fs::copy(pres_p_s.clone(), pres_pps.clone()) {
          Ok(_) => {
            println!("Launching default app for file '{}'", pres_pps.clone());
            let mut child = Command::new("cmd.exe")
              .arg("/C")
              .arg(pres_pps.clone())
              .spawn()
              .expect("Failed to execute default app");
            
            // Sleep for 2.5 seconds first
            thread::sleep(Duration::from_millis(2500));
            
            // Loops
            kill_child_when_file_moves(&mut child, pres_p, "POWERPNT.EXE");
          }
          Err(e) => {
            println!("{}", e);
          }
        }
      }
    }
  }
  #[cfg(target_family = "unix")]
  {
    { // PDF
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
    { // PPTX
      let pres_p_s = format!("{}/cartridge-pres.pptx", usb_root);
      println!("pres_p_s={}", pres_p_s);
      let pres_p = Path::new(&pres_p_s);
      if pres_p.exists() {
        println!("Launching libreoffice --show '{}'", pres_p_s);
        let mut child = Command::new("libreoffice") // TODO check if installed first
          .arg("--invisible")
          .arg("--norestore")
          .arg("--show")
          .arg(pres_p_s.clone())
          .spawn()
          .expect("Failed to execute libreoffice");
        
        // Loops, pkills soffice.bin
        kill_child_when_file_moves(&mut child, pres_p, "soffice.bin");
        
      }
    }
    { // PPT
      let pres_p_s = format!("{}/cartridge-pres.ppt", usb_root);
      println!("pres_p_s={}", pres_p_s);
      let pres_p = Path::new(&pres_p_s);
      if pres_p.exists() {
        println!("Launching libreoffice --show '{}'", pres_p_s);
        let mut child = Command::new("libreoffice") // TODO check if installed first
          .arg("--invisible")
          .arg("--norestore")
          .arg("--show")
          .arg(pres_p_s.clone())
          .spawn()
          .expect("Failed to execute libreoffice");
        
        // Loops, pkills soffice.bin
        kill_child_when_file_moves(&mut child, pres_p, "soffice.bin");
        
      }
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
        println!("Done with {}! (status={})", child_name, status);
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
      // We observe the above does not always kill procs;
      // in particular libreoffice
      #[cfg(target_family = "unix")]
      {
        Command::new("pkill") // TODO check if installed first
          .arg(child_name.clone())
          .spawn()
          .expect("Failed to execute pkill");
      }
      // Likewise on windows with powerpoint.exe
      #[cfg(target_family = "windows")]
      {
        Command::new("taskkill")
          .arg("/F")
          .arg("/IM")
          .arg(child_name.clone())
          .spawn()
          .expect("Failed to execute taskkill");
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

