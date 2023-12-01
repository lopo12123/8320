// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod unit_test {
    use sysinfo::{CpuExt, CpuRefreshKind, NetworkExt, RefreshKind, SystemExt};

    #[test]
    fn get_info() {
        let mut info = sysinfo::System::new_with_specifics(RefreshKind::new().with_networks());
        info.refresh_networks();

        let network = info.networks();
        for (interface_name, data) in network {
            println!(
                "[{}] in: {}, out: {}",
                interface_name,
                data.received(),
                data.transmitted(),
            );
        }
    }
}
