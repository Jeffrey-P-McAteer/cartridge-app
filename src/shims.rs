
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
  use std::os::raw::c_char;
  use std::ffi::{CStr,CString};
  
  let mut data_buffer: Vec<u8> = vec![0; 512]; // 512 zeroes
  
  let s = CString::new(data_buffer).unwrap();
  let raw_s = s.into_raw();
  
  unsafe {
    winapi::um::libloaderapi::GetModuleFileNameW(mem::zeroed(), raw_s as *mut u16, 510);
  }
  
  let s = unsafe { CString::from_raw(raw_s) };
  let rust_string = s.to_string_lossy().to_string();
  
  println!("get_path_to_self_exe returns '{}'", rust_string);
  return rust_string;
}
