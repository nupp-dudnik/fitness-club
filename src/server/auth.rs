use actix_web::{
  body::MessageBody,
  dev::{ServiceRequest, ServiceResponse},
  http::header::AUTHORIZATION,
  middleware::Next,
  Error, HttpMessage,
};
use uuid::Uuid;

use crate::auth::{claims::Claims, user::AuthUser};

pub async fn auth_middleware(
  req: ServiceRequest,
  next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
  if let Some(user_id) = extract_user_id(&req) {
    req.extensions_mut().insert(AuthUser { user_id });
  }
  next.call(req).await
}

fn extract_user_id(req: &ServiceRequest) -> Option<Uuid> {
  let auth_header = req.headers().get(AUTHORIZATION)?;
  let value = auth_header.to_str().ok()?;
  let (_, jwt) = value.split_once("Bearer ")?;
  let claims = Claims::decode(jwt).ok()?;

  Uuid::parse_str(&claims.sub).ok()
}
