use std::{fs::File, io::{Write, Read}};

use aliyun_oss_client::{
  plugin::Plugin, 
  client::Client
};
use serde::{Deserialize, Serialize};
use tauri::{State, api::path::data_dir};
use std::{
  sync::{
    Arc, Mutex,
  },
};

#[derive(Default,Serialize, Deserialize, Clone)]
pub struct OssConfig{
  pub key_id: String,
  pub key_secret: String,
  pub endpoint: String,
  pub bucket: String,
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

  pub fn get_bucket_domain(&self) -> String {
    let bucket = String::from("https://") + &self.bucket + ".";
    let endpoint = self.endpoint.replace("https://", &bucket);
    endpoint
  }

  // pub fn get_version_file_url(&self) -> String {
  //   self.get_bucket_domain() + &self.version_file
  // }

  pub fn get_file_url(&self, path: String) -> String {
    self.get_bucket_domain() + "/" + &path
  }
}


#[derive(Default)]
pub struct OssConfigWrapper(Arc<Mutex<OssConfig>>);

#[tauri::command]
pub fn save_oss_config(config: OssConfig) -> Result<String, String> {
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

pub struct FileType;

impl Plugin for FileType {
  fn name(&self) -> &'static str {
    "sig_file_ext"
  }

  fn initialize(&mut self, client: &mut aliyun_oss_client::client::Client) -> aliyun_oss_client::errors::OssResult<()> {
    let sig_mime_type = "application/pgp-signature";
    let sig_extension = "sig";
    fn sig_match(buf: &[u8]) -> bool {
      return buf.len() >= 3 && buf[0] == 0x64 && buf[1] == 0x57 && buf[2] == 0x35;
    }
    client.infer.add(sig_mime_type, sig_extension, sig_match);

    let json_mime_type = "application/json";
    let json_extension = "json";
    fn json_match(buf: &[u8]) -> bool {
        return buf.len() >= 3 && buf[0] == 0x7b
    }
    client.infer.add(json_mime_type, json_extension, json_match);

    Ok(())
  }
}

#[derive(Default)]
pub struct OssState<'a>{
  pub client: Mutex<Option<Client<'a>>>,
}

// #[tauri::command]
// pub fn init_oss<'a>(oss_state: State<'a, OssState>) -> Result<String, String> {
//     let config = OssConfig::from_file().unwrap();
//     // if let Err(_) = config {
//     //     let oss_state = OssState::default();
//     // };

//     let key_id = Cow::from(config.key_id.clone());
//     let key_secret = &config.key_secret;
//     let endpoint = &config.endpoint;
//     let bucket = &config.bucket;
//     let client = aliyun_oss_client::client_cow(key_id,key_secret, endpoint, bucket)
//         //.plugin(Box::new(SigFile{})).map_err(|e|e.to_string()).unwrap()
//         ;
//     // let state = OssState{
//     //     client: Mutex::new(Some(client)),
//     // };
//     *oss_state.client.lock().unwrap() = Some(client);
//     Ok("ok".into())
// }

#[tauri::command]
pub async fn upload_files(files: Vec<String>, app_index: usize) -> Result<String, String> {
  use super::app::AppList;
  use std::path::Path;

  let app = AppList::get_all()?.get(app_index)?;

  let config = OssConfig::from_file()?;

  let client = aliyun_oss_client::client(&config.key_id,&config.key_secret, &config.endpoint, &config.bucket)
    .plugin(Box::new(FileType{})).map_err(|e|e.to_string())?
    ;

  for file in files.into_iter() {
    let file_str = app.path.to_owned() + "/" + &file;
    let file_name = Path::new(&file_str);
    let key = app.oss_path.to_owned() + "/" + file.as_ref();
    let _result = client.put_file(file_name.to_owned(), &key).await.map_err(|e|e.to_string())?;
  }

  Ok("ok".into())
}

// debug 可以删掉
#[derive(Serialize, Deserialize, Debug)]
pub struct Publish{
    notes: String,
    pub_date: String,
    files: Vec<String>,
    version: String,
    signature: String,
    app_index: usize,
}


// 发布版本
#[tauri::command]
pub async fn publish(info: Publish) -> Result<String, String> {
    use super::app::AppList;
    use std::path::Path;
    use serde_json::{json, to_writer_pretty};

    let app = AppList::get_all()?.get(info.app_index)?;

    let config = OssConfig::from_file()?;

    let client = aliyun_oss_client::client(&config.key_id,&config.key_secret, &config.endpoint, &config.bucket)
        .plugin(Box::new(FileType{})).map_err(|e|e.to_string())?
        ;

    // TODO 需要改成可复用的
    for file in info.files.clone().into_iter() {
        let file_str = app.path.to_owned() + "/" + &file;
        let file_name = Path::new(&file_str);
        let key = app.oss_path.to_owned() + "/" + file.as_ref();
        let _result = client.put_file(file_name.to_owned(), &key).await.map_err(|e|e.to_string())?;
    }

    let mut zip_file = info.files.into_iter().filter(|f|{ 
        f.ends_with("zip")
    });
    let zip_file = zip_file.next();
    if let None = zip_file {
        return Err("not found zip file".into());
    }
    let zip_file = app.oss_path + &zip_file.unwrap();
    let zip_file_url = config.get_file_url(zip_file);

    let json = json!({
        "url": zip_file_url,
        "version": info.version,
        "notes": info.notes,
        "pub_date": info.pub_date,
        "signature": info.signature,
    });

    let mut json_vu8 = Vec::new();
    to_writer_pretty(&mut json_vu8, &json).map_err(|e|e.to_string())?;

    client.put_content(&json_vu8, &app.version_file).await.map_err(|e|e.to_string())?;

    Ok("ok".into())
}
