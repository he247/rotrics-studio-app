use rotrics_studio_app::serial_port::SerialPortManager;

/// Tauri command used by the frontend to query currently available serial ports.
#[tauri::command]
fn list_ports() -> Vec<String> {
    SerialPortManager::list_available_ports()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![list_ports])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
