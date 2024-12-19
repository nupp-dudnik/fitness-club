use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Staff {
  pub id: Uuid,
  pub location: Uuid,
  pub user: Uuid,
  pub role: StaffRole,
  pub hourly_rate: i32,
  pub joined_at: DateTime<Utc>,
  pub left_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum StaffRole {
  Admin,
  HumanResources,
  Manager,
  Trainer,
  Cleaner,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct StaffHours {
  pub id: Uuid,
  pub staff: Uuid,
  pub clocked_in: DateTime<Utc>,
  pub clocked_out: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct StaffTask {
  pub id: Uuid,
  pub hours: Uuid,
  pub task: Uuid,
  pub memo: String,
  pub started_at: DateTime<Utc>,
  pub ended_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Task {
  pub id: Uuid,
  pub name: String,
  pub description: Option<String>,
}
