
pub fn os_main() {
  // Windows doesn't exactly have a stable temp file API
  // and I'm not going to invent one, we'll just dump the icon wherever we currently are.
  crate::make_tray( "icon.png".to_string() );
}

