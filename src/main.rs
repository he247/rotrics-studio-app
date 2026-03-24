mod serial_port;
mod gcode_parser;

#[tauri::command]
fn list_ports() -> Vec<String> {
    serial_port::SerialPortManager::list_available_ports()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![list_ports])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}