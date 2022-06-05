use tauri::App;
use tauri::Manager;

pub fn init_window(app:&mut App) -> Result<(), Box<dyn std::error::Error>> {
    let window = app.get_window("main").unwrap();
    window.hide().unwrap();
    Ok(())
}
