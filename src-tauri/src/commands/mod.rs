use local_ip_address::local_ip;

#[tauri::command]
pub fn find_my_ip() -> String {
    let my_local_ip = local_ip().unwrap();
    format!("{}", my_local_ip) }
