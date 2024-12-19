use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct MemberVisit {
  pub id: Uuid,
  pub user: Uuid,
  pub membership: Uuid,
  pub location: Uuid,
  pub visit_time: DateTime<Utc>,
  pub leave_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Membership {
  pub id: Uuid,
  pub owner: Uuid,
  pub created_at: DateTime<Utc>,
  pub expire_at: DateTime<Utc>,
  #[serde(rename = "type")]
  pub membership_type: Uuid,
  pub renew: bool,
  pub payment_method: PaymentMethod,
  pub join_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Type)]
pub enum PaymentMethod {
  Cash,
  Credit,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Subscription {
  pub id: Uuid,
  pub access_level: AccessLevel,
  pub price: i32,
  pub period: Period,
  pub custom_period: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubscriptionPublic {
  pub access_level: AccessLevel,
  pub price: i32,
  pub period: Period,
  pub custom_period: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
pub enum AccessLevel {
  Premium,
  Family,
  EarlyBird,
  Basic,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
pub enum Period {
  Yearly,
  Monthly,
  Weekly,
  Onetime,
}
