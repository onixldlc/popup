use tauri::{GlobalWindowEvent, WindowEvent};
// use tauri::AppHandle;

pub fn window_event_handler(global:GlobalWindowEvent){
    let event = global.event();
    let window = global.window();
    match event{
        WindowEvent::CloseRequested {api, ..} => {
            api.prevent_close();
            window.hide().unwrap();
        }
        _ => {}
    }
}