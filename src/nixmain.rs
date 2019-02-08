use mktemp;

use std::io::prelude::*;

pub fn os_main() {
  let icon_tmp_f = extract_icon();
  match icon_tmp_f {
    Some(icon_tmp_f) => {
      crate::make_tray( format!("{}", icon_tmp_f.path()) );
    }
    None => {
      crate::make_tray( "".to_string() );
    }
  }
}


// returns full path to icon
fn extract_icon() -> Option<mktemp::TempFile> {
  let icon_bytes = include_bytes!("../icon.png");
  match mktemp::TempFile::new("icon", ".png") {
    Ok(mut temp_file) => {
      match temp_file.inner().write_all(icon_bytes) {
        Ok(_) => { }
        Err(e) => {
          println!("{}", e);
          return None;
        }
      }
      return Some(temp_file);
    }
    Err(e) => {
      println!("{}", e);
      return None;
    }
  }
}
