#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod oss;
use std::sync::{Mutex};

use oss::{save_oss_config,get_oss_config, upload_files, publish, OssConfigWrapper, OssConfig};

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
        .menu(
            tauri::Menu::os_default("Tauri Publish App")
        )
        //.manage(OssConfigWrapper { db: Default::default() })
        .setup(|app|{
            if let Ok(config) = OssConfig::from_file(){
                app.manage(OssConfigWrapper { db: Mutex::new(config)});
            }else{
                app.manage(OssConfigWrapper { db: Default::default() });
            }

            Ok(())
        })
        .run(ctx)
        .expect("error while running tauri application");
}
