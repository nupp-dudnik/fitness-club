use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Login {
  pub email: String,
  pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserInfo {
  pub name: String,
  pub surname: Option<String>,
  pub email: String,
  pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Token {
  pub token: String,
}
