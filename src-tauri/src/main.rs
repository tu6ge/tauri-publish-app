#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod oss;
use oss::{get_oss_config, publish, save_oss_config, upload_files, OssConfigWrapper};
mod app;
use app::{app_check_oss, get_all_app, get_app, push_app, remove_app, update_app};
use tauri::Manager;

fn main() {
    let ctx = tauri::generate_context!();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            save_oss_config,
            get_oss_config,
            upload_files,
            publish,
            app_check_oss,
            get_all_app,
            get_app,
            push_app,
            remove_app,
            update_app,
        ])
        .menu(tauri::Menu::os_default("Tauri Publish App"))
        //.manage(OssConfigWrapper { db: Default::default() })
        .setup(|app| {
            app.manage(OssConfigWrapper::from_file());

            Ok(())
        })
        .run(ctx)
        .expect("error while running tauri application");
}
