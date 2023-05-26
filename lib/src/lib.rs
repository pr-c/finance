use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Book {
    pub name: String,
    pub description: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct Currency {
    pub symbol: String,
    pub description: Option<String>,
    pub decimal_points: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub name: String,
    pub description: Option<String>
}

impl IntoResponse for Currency {
    fn into_response(self) -> Response {
        object_to_response(&self)
    }
}

impl IntoResponse for Book {
    fn into_response(self) -> Response {
        object_to_response(&self)
    }
}

fn object_to_response<T: Serialize>(t: &T) -> Response {
    if let Ok(serialized) = serde_json::to_string(t) {
        serialized.into_response()
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR).into_response()
    }
}
