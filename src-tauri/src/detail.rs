use tauri::api::path::config_dir;
use std::{path::PathBuf, fs, io::Write};
use serde::{Serialize, Deserialize};
use std::sync::Mutex;

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Detail {
  name: String,
  url: String,
  list: Vec<String>,
}
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct  Wraper {
    vector: Vec<Detail>,
    index: usize
}

pub struct Vector {
  vector: Mutex<Vec<Detail>>,
  index: Mutex<usize>,
}

impl Vector {
    pub fn new() -> Self {
      Self { 
        vector: Mutex::new(read_json()),
        index: Mutex::new(0),
      }
    }

    pub fn update_index(&self, i:usize){
        let mut index = self.index.lock().unwrap();
        *index = i;
    }

    pub fn add_new_detail(&self, name:String){
        let mut list = self.vector.lock().unwrap();
        let new_detail = Self::new_detail(name);
        list.push(new_detail);
        self.update_index(list.len() - 1)
    }
    pub fn detail_list(&self) -> Wraper{
        let list = self.vector.lock().unwrap().clone();
        let index = self.index.lock().unwrap().clone();
        let mut dir_path = config_dir().unwrap();
        dir_path.push("Oshipy");
        match fs::create_dir(dir_path) {
            Ok(_)=>0,
            Err(_)=>1,
        };
        let mut file = match fs::File::create(get_dir()) {
            Err(why)=>panic!("couldn't create: {}", why),
            Ok(file)=>file,
        };
        let serialized = serde_json::to_string(&list).unwrap();
        match file.write_all(serialized.as_bytes()) {
            Err(why)=>panic!("couldn't write: {}", why),
            Ok(_)=>0,
        };
        Wraper { vector: list, index: index }
    }
    pub fn delete_detail(&self){
        let mut index = self.index.lock().unwrap();
        let i = index.clone();
        let mut list = self.vector.lock().unwrap();
        list.remove(i);
        *index = 0;
    }

    pub fn add_text(&self, str:String){
        let index = self.index.lock().unwrap().clone();
        let mut vect = self.vector.lock().unwrap();
        vect[index].list.push(str);
    }

    pub fn delete_text(&self, i:usize){
        let index = self.index.lock().unwrap().clone();
        let mut vect = self.vector.lock().unwrap();
        vect[index].list.remove(i);
    }

    fn new_detail(str:String) -> Detail {
        let name = str;
        let list:Vec<String> = Vec::new();
        let url = String::new();

        Detail { name, url, list }
    }
}

pub mod commands {

    use tauri::State;
    use super::*;
      
    #[tauri::command]
    pub fn read_data(detail: State<'_, Vector>,) -> Wraper{
        detail.detail_list()
    }
    #[tauri::command]
    pub fn selected_index(detail: State<'_, Vector>, index: usize){
        detail.update_index(index);
    }
    #[tauri::command]
    pub fn remove_obj(detail: State<'_, Vector>){
        detail.delete_detail();
    }
    #[tauri::command]
    pub fn add_obj(detail: State<'_, Vector>, name: String){
        detail.add_new_detail(name);
    }
    #[tauri::command]
    pub fn add_list_text(detail: State<'_, Vector>, text: String){
        detail.add_text(text);
    }
    #[tauri::command]
    pub fn remove_list_text(detail: State<'_, Vector>, index: usize){
        detail.delete_text(index);
    }
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
fn read_json() -> Vec<Detail>{
    let data = fs::read_to_string(get_dir());
    let data = match data {
        Err(_) => String::from(r#"[]"#),
        Ok(d) => d
    };
    let deserialized: Vec<Detail> = serde_json::from_str(&data).unwrap();
    deserialized
}

