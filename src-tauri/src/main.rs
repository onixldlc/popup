#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
// use sysinfo::{ProcessExt, System, SystemExt};

mod tray_handler;
mod window_handler;
mod sleep_handler;
mod init;

fn main() {
  println!("hello there"); 

  sleep_handler::get_app_time();
  let system_tray = tray_handler::tray_menu_handler();

  tauri::Builder::default()
    .system_tray(system_tray)
    .on_system_tray_event(tray_handler::tray_event_handler)
    .on_window_event(window_handler::window_event_handler)
    .setup(init::init_window)
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

  // app.new()
}
