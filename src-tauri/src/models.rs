use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TodoItem {
  pub id: String,
  
  // display metadata
  pub title: String,
  pub description: Option<String>,
  pub priority: u64,

  // grouping metadata
  pub group_id: Option<String>,

  // time related attributes
  pub completed_at: Option<u64>,
  pub created_at: u64,
  pub due_at: Option<u64>,
  pub remind_at: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoGroup {
  pub id: String,
  
  // display metadata
  pub name: String,
  pub color: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoData {
  pub items: Vec<TodoItem>,
  pub groups: Vec<TodoGroup>,
}