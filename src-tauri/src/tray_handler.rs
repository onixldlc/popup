use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem, SystemTrayEvent};
use tauri::Manager;
use tauri::AppHandle;

pub fn tray_menu_handler() -> SystemTray {
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let hide = CustomMenuItem::new("hide".to_string(), "Hide");
  let tray_menu = SystemTrayMenu::new()
    .add_item(quit)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(hide);
  let system_tray = SystemTray::new()
    .with_menu(tray_menu);
  return system_tray;
}

pub fn tray_event_handler(app:&AppHandle, event:SystemTrayEvent){
  match event {
    SystemTrayEvent::MenuItemClick { id, .. } => {
      match id.as_str() {
        "quit" => {
          std::process::exit(0);
        }
        "hide" => {
          let window = app.get_window("main").unwrap();
          window.hide().unwrap();
        }
        _ => {}
      }
    }

    SystemTrayEvent::DoubleClick {..} => {
      let window = app.get_window("main").unwrap();
      window.show().unwrap();
      if let Err(_err)=window.set_focus(){
        println!("somethign went wrong when focussing the window");
      }
    }
    
    _ => {}
  }
}