#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod oss;
use oss::{save_oss_config,get_oss_config, upload_files, OssState, publish};

mod app;
use app::{AppList, get_all_app, get_app, push_app, remove_app, update_app};
use tauri::Manager;

fn main() {

    let ctx = tauri::generate_context!();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            save_oss_config,
            get_oss_config,
            upload_files,
            publish,
            get_all_app,
            get_app,
            push_app,
            remove_app,
            update_app,
        ])
        .menu(
            tauri::Menu::os_default("Tauri Publish App")
        )
        .setup(|app|{
            app.manage(OssState::default());
            app.manage(AppList::default());
            Ok(())
        })
        .run(ctx)
        .expect("error while running tauri application");
}
