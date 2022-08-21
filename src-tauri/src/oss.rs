use std::{fs::File, io::{Write, Read}};

use aliyun_oss_client::plugin::Plugin;
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

impl OssConfig {
  pub fn from_file() -> Result<Self, String> {
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
  OssConfig::from_file()
}

struct SigFile;

impl Plugin for SigFile {
  fn name(&self) -> &'static str {
    "sig_file_ext"
  }

  fn initialize(&mut self, client: &mut aliyun_oss_client::client::Client) -> aliyun_oss_client::errors::OssResult<()> {
    let mime_type = "application/pgp-signature";
    let extension = "sig";
    fn m(buf: &[u8]) -> bool {
      return buf.len() >= 3 && buf[0] == 0x64 && buf[1] == 0x57 && buf[2] == 0x35;
    }
    client.infer.add(mime_type, extension, m);

    Ok(())
  }
}

#[tauri::command]
pub async fn upload_files(files: Vec<String>, app_index: usize) -> Result<String, String> {
  use super::app::AppList;
  use std::path::Path;

  let app = AppList::get_all()?.get(app_index)?;

  let config = OssConfig::from_file()?;

  let client = aliyun_oss_client::client(&config.key_id,&config.key_secret, &config.endpoint, &config.bucket)
    .plugin(Box::new(SigFile{})).map_err(|e|e.to_string())?
    ;

  for file in files.into_iter() {
    let file_str = app.path.to_owned() + "/" + &file;
    println!("filename {}",file_str);
    let file_name = Path::new(&file_str);
    let key = config.save_path.to_owned() + "/" + file.as_ref();
    println!("key {}",key);
    let result = client.put_file(file_name.to_owned(), &key).await.map_err(|e|e.to_string())?;
    println!("result: {}",result);
  }

  Ok("ok".into())
}