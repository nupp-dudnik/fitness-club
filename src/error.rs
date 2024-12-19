use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
  #[error("500 Internal Server Error\n\n{0}")]
  InternalError(#[from] anyhow::Error),
  #[error("User does not exist")]
  UserDoesNotExist,
  #[error("Email already registered")]
  EmailUsed,
  #[error("Invalid password")]
  InvalidPassword,
}

impl ResponseError for Error {
  fn status_code(&self) -> StatusCode {
    match &self {
      Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Self::UserDoesNotExist => StatusCode::UNAUTHORIZED,
      Self::InvalidPassword => StatusCode::UNAUTHORIZED,
      Self::EmailUsed => StatusCode::UNAUTHORIZED,
    }
  }

  fn error_response(&self) -> HttpResponse {
    HttpResponse::build(self.status_code()).body(self.to_string())
  }
}

pub type Result<T> = std::result::Result<T, Error>;
