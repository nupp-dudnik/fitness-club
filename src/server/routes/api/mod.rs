mod auth;
mod membership;
mod user;

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
  cfg
    .service(auth::login)
    .service(auth::register)
    .service(auth::refresh)
    .service(user::get_self)
    .service(user::get_other)
    .service(user::update_self)
    .service(user::update_other)
    .service(membership::create_new)
    .service(membership::get_available)
    .service(membership::get_self)
    .service(membership::get_other);
}
