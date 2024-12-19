use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Tool {
  pub id: Uuid,
  pub name: String,
  pub kind: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct ToolAtLocation {
  pub id: Uuid,
  pub tool: Uuid,
  pub description: Option<String>,
  pub condition: Option<String>,
}
