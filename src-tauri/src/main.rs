#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod oss;
use oss::{OssConfigWrapper,save_oss_config,get_oss_config};

fn main() {
    let ctx = tauri::generate_context!();
    tauri::Builder::default()
        .manage(OssConfigWrapper::default())
        .invoke_handler(tauri::generate_handler![save_oss_config,get_oss_config])
        .menu(
            tauri::Menu::os_default("Tauri Vue Template")
        )
        .run(ctx)
        .expect("error while running tauri application");
}
