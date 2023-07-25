use axum::http::StatusCode;
use axum::{Json, extract::State};
use diesel::RunQueryDsl;
use uuid::Uuid;

use sorry_youth_imagine::models::Images;
use sorry_youth_imagine::schema::images;
use serde::{Serialize, Deserialize};
use chrono::prelude::*;

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
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<Images>), (StatusCode, String)> {
    // insert your application logic here
    let new_user = Images {
        id: Uuid::now_v7(),
        date: Utc::now().naive_utc(),
        upload_by: Default::default(),
        md5: "".to_string(),
        origin_filename: "".to_string(),
    };
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(|conn| {
            diesel::insert_into(images::table)
                .values(new_user)
                .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    Ok((StatusCode::CREATED, Json(res)))
}


