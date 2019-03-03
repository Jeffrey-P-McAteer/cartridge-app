
#[cfg(target_family = "unix")]
pub fn get_path_to_self_exe() -> String {
  use std::fs;
  use std::env;
  match fs::read_link("/proc/self/exe") {
    Ok(path) => {
      path.as_path().to_string_lossy().to_string()
    }
    Err(e) => {
      println!("{}", e);
      if let Some(arg0) = env::args().nth(0) {
        arg0
      }
      else {
        String::new()
      }
    }
  }
}


#[cfg(target_family = "windows")]
pub fn get_path_to_self_exe() -> String {
  use std::mem;
  use crate::winapi;
  
  winapi::um::libloaderapi::GetModuleFileNameW(mem::zeroed(), )
  
}
