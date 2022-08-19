use std::{fs::File, io::{Write, Read}};

use serde::{Deserialize, Serialize};
use tauri::{State, api::path::data_dir};
use std::{
  sync::{
    Arc, Mutex,
  },
};


#[derive(Default,Serialize, Deserialize, Clone)]
pub struct OssConfig{
  key_id: String,
  key_secret: String,
  endpoint: String,
  bucket: String,
  save_path: String,
  version_file: String,
}


#[derive(Default)]
pub struct OssConfigWrapper(Arc<Mutex<OssConfig>>);

#[tauri::command]
pub fn save_oss_config(config: OssConfig, _db: State<'_, OssConfigWrapper>) -> Result<String, String> {
  let json = serde_json::to_string_pretty(&config).map_err(|_|{"转 json 格式失败".to_string()})?;
  let data_dir = data_dir();
  if let None = data_dir {
    return Err("找不到存储路径".into());
  }

  let mut file = File::create(
      data_dir.unwrap()
      .join("tauri_oss_config.json")
    )
    .map_err(|_|"create tauri_oss_config.json failed".to_string())?;
  file.write_fmt(format_args!("{}", json)).map_err(|_|"writing tauri_oss_config.json failed".to_string())?;

  // let mut oss_config = *Arc::clone(&db.0).get_mut().unwrap();

  // oss_config.copy(config.clone());
  
  Ok("ok".into())
}

#[tauri::command]
pub fn get_oss_config() -> Result<OssConfig, String> {

  let mut content = String::new();
  let data_dir = data_dir();
  if let None = data_dir {
    return Err("找不到存储路径".into());
  }

  let mut file = File::open(
    data_dir.unwrap()
    .join("tauri_oss_config.json")
  )
  .map_err(|_|"open tauri_oss_config.json failed".to_string())?;

  file.read_to_string(&mut content).map_err(|_|"reading tauri_oss_config.json failed".to_string())?;
  let store_config: OssConfig = serde_json::from_str(&content).map_err(|_|"json parse failed".to_string())?;
  //println!("key_id: {}", store_config.key_id);

  Ok(store_config)
}