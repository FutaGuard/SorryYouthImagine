use axum::http::StatusCode;
use axum::{Json, extract::State};
use uuid::Uuid;

use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use ::entity::{images as Images};

#[derive(Serialize, Deserialize)]
pub struct CreateUser {
    token: String
}

pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

pub async fn upload_image(
    // State(pool): State<>,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<Images::Model>), (StatusCode, String)> {
    // insert your application logic here
    todo!();

}


