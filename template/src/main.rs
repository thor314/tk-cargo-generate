#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]

mod error;
#[cfg(test)] mod tests;
mod utils;
{% if cli %} mod cli; {% endif %}
{% if async -%} use tracing::info;
{% else -%} use log::info;
{% endif %}
use error::MyError;

{% if server -%}
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

async fn hello_world() -> &'static str { "Hello, world!" }

async fn error_handler() -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
}

#[shuttle_runtime::main]
async fn main( #[shuttle_secrets::Secrets] secret_store: shuttle_secrets::SecretStore,) -> shuttle_axum::ShuttleAxum {
{% else -%}
  {% if async -%} #[tokio::main] async {% endif %} fn main() -> Result<(), MyError> {
{% endif -%}
  {% if cli %} let _cli = {% endif %}
  utils::setup( {% if server -%} secret_store).unwrap(); {% else %} )?; {% endif %}

  info!("hello thor"); 

  {% if server -%}
  let router = Router::new()
    .route("/", axum::routing::get(hello_world))
    .route("/-1/error", get(error_handler));

  Ok(router.into())
  {% else -%} 
  Ok(()) 
  {% endif -%}
}
