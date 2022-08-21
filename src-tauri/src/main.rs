#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use tauri::api::path::config_dir;
use std::{path::PathBuf, fs};
fn main() {
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
  match data {
      Err(_) => String::from(r#"{}"#),
      Ok(d) => d
  }
}

#[tauri::command]
fn get_json() -> String {
  read_json(get_dir())
}
#[tauri::command]
fn save_json(data: String){
  println!("{}", data)
}