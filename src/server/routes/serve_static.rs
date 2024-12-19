use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::{get, HttpRequest};
use anyhow::Context;

use crate::error::Result;

static STATIC_FILES_DIR: &str = "./static";

#[get("/{path:.*}")]
async fn serve_static(req: HttpRequest) -> Result<NamedFile> {
  let mut path = req.match_info().query("path").to_string();
  if path.is_empty() {
    path = "index".into();
  }
  let mut path = PathBuf::from(STATIC_FILES_DIR).join(path);
  if path.extension().is_none() {
    path.set_extension("html");
  }
  if !path.exists() {
    path = PathBuf::from(STATIC_FILES_DIR).join("index.html");
  }
  Ok(
    NamedFile::open_async(path)
      .await
      .context("Failed reading the file")?,
  )
}
