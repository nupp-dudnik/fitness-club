use actix_web::middleware::from_fn;

use super::auth::auth_middleware;

mod api;
mod serve_static;

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
  cfg
    .service(
      actix_web::web::scope("/api")
        .wrap(from_fn(auth_middleware))
        .configure(api::configure),
    )
    .service(serve_static::serve_static);
}
