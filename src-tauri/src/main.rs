#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use tauri::api::path::config_dir;
use std::{path::PathBuf, fs};
use serde::{Serialize, Deserialize};
use once_cell::sync::OnceCell;

static obj: OnceCell<Vec<Detail>> = OnceCell::new();

#[derive(Serialize,Deserialize,Debug)]
pub struct Detail {
  name: String,
  list: Vec<String>,
  url: String,
}

fn main() {
  let test = read_json(get_dir());
  let mut deserialized: Vec<Detail> = serde_json::from_str(&test).unwrap();
  println!("{:?}", deserialized);
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      get_json,
      save_json
      ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn get_dir() -> PathBuf{
  let d = config_dir();
  match d {
      None => PathBuf::new(),
      Some(t) => {
        let mut dir = t;
        dir.push("Oshipy");
        dir.push("config");
        dir.set_extension("json");
        dir
    }
  }
}
fn read_json(path: PathBuf) -> String{
  let data = fs::read_to_string(path);
  let data = match data {
      Err(_) => String::from(r#"{}"#),
      Ok(d) => d
  };
  let deserialized: Vec<Detail> = serde_json::from_str(&data).unwrap();
  obj.set(deserialized).unwrap();
  println!("{:?}", obj);
  data
}

#[tauri::command]
fn get_json() -> String {
  read_json(get_dir())
}
#[tauri::command]
fn save_json(data: String){
  println!("{}", data)
}