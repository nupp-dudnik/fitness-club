use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Location {
  pub id: Uuid,
  pub country: String,
  pub city: String,
  pub street: String,
  pub working_hours: Option<String>,
}
