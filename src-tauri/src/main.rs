#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::path::PathBuf;

use directories::BaseDirs;
use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Serialize, Deserialize)]
enum TodoItemPriority {
  P1, P2, P3, P4
}

#[derive(Debug, Serialize, Deserialize)]
struct TodoItem {
  id: String,
  
  // display metadata
  title: String,
  description: Option<String>,
  priority: TodoItemPriority,

  // grouping metadata
  group_id: Option<String>,

  // time related attributes
  completed_at: Option<u64>,
  created_at: u64,
  due_at: Option<u64>,
  remind_at: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TodoGroup {
  id: String,
  
  // display metadata
  name: String,
  color: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TodoData {
  items: Vec<TodoItem>,
  groups: Vec<TodoGroup>,
}

#[tauri::command]
async fn get_data() -> Result<Vec<TodoItem>, String> {
  let file = match File::open("/Users/tyler.snedigar/repos/tydo/test-data.json") {
    Ok(f) => f,
    Err(e) => return Err(e.to_string())
  };
  let reader = BufReader::new(file);
  match serde_json::from_reader(reader) {
    Ok(t) => return t,
    Err(e) => return Err(e.to_string())
  }
}

#[tauri::command]
async fn save_data(data: TodoData) -> Result<(), String> {
  let base_dirs = BaseDirs::new().unwrap();
  let data_dir = base_dirs.data_local_dir();
  let data_file = data_dir.join("free-do").join("data.json");
  match save_data_to_file(data_file, data) {
    Ok(_) => Ok(()),
    Err(e) => Err(e.to_string())
  }
}

fn save_data_to_file(path: PathBuf, data: TodoData) -> Result<(), Error> {
  let file = File::create(path).expect("failed to open data file");
  serde_json::to_writer(file, &data)?;
  Ok(())
}

fn ensure_save_file() -> Result<(), Error> {
  let base_dirs = BaseDirs::new().unwrap();
  let data_dir = base_dirs.data_local_dir();
  if !data_dir.join("free-do").exists() {
    std::fs::create_dir(data_dir.join("free-do"))?;
  }

  let data_file = data_dir.join("free-do").join("data.json");
  if !data_file.exists() {
    let todo_data = TodoData{ items: vec![], groups: vec![] };
    save_data_to_file(data_file, todo_data)?;
  }

  Ok(())
}

fn main() {
  ensure_save_file().expect("Failed to write save file.");
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_data, save_data])
    .run(tauri::generate_context!())
    .expect("error while running free-do");
}
