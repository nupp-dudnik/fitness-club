use anyhow::{Context, Result};
use sqlx::PgPool;

pub fn connect(url: &str) -> Result<PgPool> {
  PgPool::connect_lazy(url).context("Failed to connect to the database")
}
