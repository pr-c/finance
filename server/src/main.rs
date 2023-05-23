mod engine;

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::net::SocketAddr;
use axum::extract::{ FromRequestParts};
use axum::{async_trait, Router};
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum_auth::AuthBearer;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    let router = Router::new()
        .route("/", get(root));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Claim {
    username: String
}
enum ClaimFailed{
    NoToken, NotAuthorized
}

impl IntoResponse for ClaimFailed {
    fn into_response(self) -> Response {
        (StatusCode::FORBIDDEN, "Authorisation failed.").into_response()
    }

}

#[async_trait]
impl<B: Send + Sync> FromRequestParts<B> for Claim {
    type Rejection = ClaimFailed;

    async fn from_request_parts(parts: &mut Parts, state: &B) -> Result<Self, Self::Rejection> {
        if let Ok(token) = AuthBearer::from_request_parts(parts, state).await {
            if let Some(username_value) = parts.headers.get("username") {
                if let Ok(username) = username_value.to_str() {
                    let e = engine::engine.lock().await;
                    if e.authenticate(username, &token.0).is_ok() {
                        return Ok(Self{username: username.to_owned()})
                    }
                }
            }

        }
        Err(ClaimFailed::NoToken)

    }
}

async fn root(AuthBearer(token): AuthBearer) -> String {
    format!("Hello, {}!", token)
}