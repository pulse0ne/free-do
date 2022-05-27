#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::fs::File;
use std::io::BufReader;

use serde::Serialize;
use serde::Deserialize;
use tauri::Runtime;

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

#[tauri::command]
async fn get_todos<R: Runtime>(_app: tauri::AppHandle<R>, _window: tauri::Window<R>) -> Result<Vec<TodoItem>, String> {
  // TODO: error handling
  let file = File::open("/Users/tyler.snedigar/repos/tydo/test-data.json").unwrap();
  let reader = BufReader::new(file);
  let todos: Vec<TodoItem> = serde_json::from_reader(reader).unwrap();

  Ok(todos)
}

#[tauri::command]
async fn get_groups<R: Runtime>(_app: tauri::AppHandle<R>, _window: tauri::Window<R>) -> Result<Vec<TodoGroup>, String> {
  // TODO:
  Ok(vec![TodoGroup { id: String::from("0"), name: String::from("Inbox"), color: String::from("dodgerblue") }])
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_todos, get_groups])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
