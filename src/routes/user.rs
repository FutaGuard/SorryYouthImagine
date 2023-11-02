use axum::http::StatusCode;
use axum::{Json, extract::State};
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection};
use uuid::Uuid;

use serde::{Serialize, Deserialize};
use ::entity::{users as Users};
use sea_orm::ActiveValue::{Set, NotSet, Unchanged};
// use entity::prelude::Users;

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



pub async fn create_user(
    State(pool): State<DatabaseConnection>,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<Users::Model>), (StatusCode, String)> {
    // insert your application logic here
    let new_user = Users::ActiveModel{
        id: Set(Uuid::now_v7()),
        is_admin: Set(false),
        active: Set(true),
        username: Set("".to_string()),
        password: Set("".to_string()),
    };
    // let conn = pool.get().await.map_err(internal_error)?;
    let new_user: Users::Model = new_user.insert(&pool).await.expect("err");

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    Ok((StatusCode::CREATED, Json(new_user)))
}


