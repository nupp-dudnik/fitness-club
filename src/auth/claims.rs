use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const TOKEN_TTL: i64 = 60 * 60;

lazy_static::lazy_static!(
  static ref JWT_SECRET: String = std::env::var("JWT_SECRET")
    .unwrap_or("deadbeef".to_string());
);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Claims {
  pub sub: String,
  iat: i64,
  exp: i64,
  aud: Vec<String>,
  iss: String,
}

impl Claims {
  pub fn decode(jwt: &str) -> Result<Self> {
    let mut validation = jsonwebtoken::Validation::default();
    validation.set_audience(&["Fitness Club"]);

    let key = jsonwebtoken::DecodingKey::from_base64_secret(&JWT_SECRET)?;
    let claims = jsonwebtoken::decode::<Self>(jwt, &key, &validation)?.claims;
    Ok(claims)
  }

  pub fn jwt(user_id: Uuid) -> Result<String> {
    let now = chrono::Utc::now();
    let claims = Self {
      sub: user_id.simple().to_string(),
      exp: now.timestamp() + TOKEN_TTL,
      iat: now.timestamp(),
      aud: vec!["Fitness Club".to_owned()],
      iss: "Fitness Club".to_owned(),
    };

    let key =
      jsonwebtoken::EncodingKey::from_base64_secret(&JWT_SECRET).context("invalid secret")?;

    let token = jsonwebtoken::encode(&Default::default(), &claims, &key).unwrap();

    Ok(token)
  }
}
