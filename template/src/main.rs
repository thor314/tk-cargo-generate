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
// use diesel_async::{
//   pooled_connection::deadpool::Pool,
//   AsyncPgConnection,
// };
use axum::{http::StatusCode, response::IntoResponse, routing::{get, post}, Router};

async fn hello_world() -> &'static str { "Hello, world!" }

async fn error_handler() -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
}

#[shuttle_runtime::main]
async fn main( #[shuttle_secrets::Secrets] secret_store: shuttle_secrets::SecretStore,
    // #[shuttle_diesel_async::Postgres] pg: Pool<AsyncPgConnection>
) -> shuttle_axum::ShuttleAxum {
{% else -%}
  {% if async -%} #[tokio::main] async {% endif %} fn main() -> Result<(), MyError> {
{% endif -%}
  {% if cli %} let cli = {% endif %}
  utils::setup( {% if server -%} &secret_store).unwrap(); {% else %} )?; {% endif %}
  {% if cli -%} cli.handle(); {% endif %}

  info!("hello thor"); 

  {% if server -%}
  let router = Router::new()
    .route("/", get(hello_world))
    .route("/-1/error", get(error_handler))
    .route("/-1/health", get(|| async { StatusCode::OK }));

  Ok(router.into())
  {% else -%} 
  Ok(()) 
  {% endif -%}
}
