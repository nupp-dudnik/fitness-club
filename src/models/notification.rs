use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Notification {
  pub id: Uuid,
  pub user: Uuid,
  pub title: String,
  pub body: String,
  pub created_at: DateTime<Utc>,
  pub read_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Warning {
  pub id: Uuid,
  pub notification: Uuid,
  pub author: Uuid,
  pub reason: String,
  pub severity: i32,
}
