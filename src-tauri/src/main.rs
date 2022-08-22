#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Manager;

use detail::Vector;
use detail::commands;
pub mod detail;



fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      commands::read_data,
      commands::selected_index,
      commands::remove_obj,
      commands::add_obj,
      commands::add_list_text,
      commands::remove_list_text,
      ])
    .setup(|app|{
      let detail = Vector::new();
      app.manage(detail);
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

