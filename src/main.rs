use anyhow::Context;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  dotenv::dotenv().ok();
  let db_url = std::env::var("DATABASE_URL").context("DATABASE_URL not set")?;
  let pool = fitness_club::db::connect::connect(&db_url)?;
  fitness_club::server::start("0.0.0.0", 7777, pool).await
}
