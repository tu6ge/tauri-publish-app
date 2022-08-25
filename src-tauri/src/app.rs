use std::{fs::File, io::{Write, Read}};

use serde::{Deserialize, Serialize};
use tauri::{api::path::data_dir};


#[derive(Default,Serialize, Deserialize, Clone)]
pub struct AppConfig{
  pub name: String,
  pub path: String,
  pub oss_path: String,
  pub version_file: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct AppList{
  list: Vec<AppConfig>,
  index: usize,
}

impl AppList {
  pub fn get_all()-> Result<Self, String>{
    let mut content = String::new();
    let data_dir = data_dir();

    let mut file = File::options().write(true).read(true).create(true).open(
      data_dir.unwrap()
      .join("tauri_oss_app_config.json")
    )
    .map_err(|_|"open tauri_oss_app_config.json failed".to_string())?;

    file.read_to_string(&mut content).map_err(|_|"reading tauri_oss_app_config.json failed".to_string())?;

    if content.is_empty() {
      return Ok(AppList {
        list: Vec::new(),
        index: 0,
      });
    }

    let list: AppList = serde_json::from_str(&content).map_err(|_|"json parse failed".to_string())?;

    Ok(list)
  }

  fn save(&self) -> Result<String, String> {
    let json = serde_json::to_string_pretty(self).map_err(|_|{"转 json 格式失败".to_string()})?;
    let data_dir = data_dir();
    if let None = data_dir {
      return Err("找不到存储路径".into());
    }

    let mut file = File::create(
        data_dir.unwrap()
        .join("tauri_oss_app_config.json")
      )
      .map_err(|_|"create tauri_oss_app_config.json failed".to_string())?;
    file.write_fmt(format_args!("{}", json)).map_err(|_|"writing tauri_oss_app_config.json failed".to_string())?;

    Ok("ok".into())
  }

  fn push(mut self, app: AppConfig)-> Self{
    self.list.push(app);
    self
  }

  fn set_index(mut self, index: usize)-> Self{
    self.index = index;
    self
  }

  fn update(mut self, index: usize, app: AppConfig)-> Self{
    self.index = index;
    self.list[index] = app;
    self
  }

  pub fn get(self, index: usize)-> Result<AppConfig, String>{
    if self.list.len() > index {
      Ok(self.list[index].clone())
    }else {
      Err("no found".into())
    }
  }

  fn remove(mut self, index: usize)-> Self{
    self.list.remove(index);
    self
  }
}

#[tauri::command]
pub fn get_all_app() -> Result<AppList, String> {
  AppList::get_all()
}

#[tauri::command]
pub fn get_app(index: usize) -> Result<AppConfig, String> {
  AppList::get_all()?.get(index)
}

#[tauri::command]
pub fn update_app(index: usize, app: AppConfig) -> Result<String, String> {
  AppList::get_all()?.update(index, app).save()
}


#[tauri::command]
pub fn push_app(app: AppConfig) -> Result<String, String> {
  let list = AppList::get_all()?;
  list.push(app).set_index(0).save()
}

#[tauri::command]
pub fn remove_app(index: usize) -> Result<String, String> {
  let list = AppList::get_all()?;
  list.remove(index).set_index(0).save()
}