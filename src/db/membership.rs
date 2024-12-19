use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{
  member::{AccessLevel, Membership, Period, Subscription},
  user::User,
};

pub async fn get_all_subs(pool: &PgPool) -> Result<Vec<Subscription>> {
  let subs = sqlx::query_as!(
      Subscription,
      "SELECT id, access_level AS \"access_level: _\", price, period AS \"period: _\", custom_period FROM subscriptions"
    )
    .fetch_all(pool)
    .await?;
  Ok(subs)
}

pub async fn get_memberships_by_user_id(pool: &PgPool, user_id: Uuid) -> Result<Vec<Membership>> {
  let memberships = sqlx::query_as!(
    Membership,
    "SELECT
      id,
      owner,
      created_at,
      expire_at,
      type AS membership_type,
      renew,
      payment_method AS \"payment_method: _\",
      join_reason
    FROM memberships
    WHERE owner = $1",
    user_id
  )
  .fetch_all(pool)
  .await?;
  Ok(memberships)
}

pub async fn create_sub(
  pool: &PgPool,
  access_level: AccessLevel,
  price: i32,
  period: Period,
  custom_period: Option<&str>,
) -> Result<Uuid> {
  let id = Uuid::new_v4();
  sqlx::query!("
    INSERT INTO subscriptions (id, access_level, price, period, custom_period) VALUES ($1, $2, $3, $4, $5)",
      id,
      access_level as _,
      price,
      period as _,
      custom_period,
  )
    .execute(pool)
    .await?;
  Ok(id)
}
