use actix_web::{get, App, HttpRequest, HttpServer, Responder};
use anyhow::Context;
use sqlx::PgPool;

use super::routes::configure;

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
  "Welcome!"
}

pub async fn start(bind_addr: &str, bind_port: u16, pg_pool: PgPool) -> anyhow::Result<()> {
  let data = actix_web::web::Data::new(pg_pool);
  HttpServer::new(move || App::new().app_data(data.clone()).configure(configure))
    .bind((bind_addr, bind_port))?
    .run()
    .await
    .context("Web server failed")
}
