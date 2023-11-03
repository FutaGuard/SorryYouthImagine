use axum::http::StatusCode;
use axum::{Json, extract::State};
use sea_orm::{ActiveModelTrait, ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use serde::{Serialize, Deserialize};
use ::entity::{users as Users};
use sea_orm::ActiveValue::{Set};
use passwords;
// use entity::prelude::Users;

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateUser {
    token: String,
    username: String,
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
    let user = Users::Entity::find()
        .filter(Users::Column::Username.eq(payload.clone().username))
        .one(&pool)
        .await;

    match user {
        Ok(data) => {
            if data.is_some() {
                return Err((StatusCode::BAD_REQUEST, "input duplicate username".to_string()));
            }
        }
        Err(_) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "Unable to connect DB".to_string()));
        }
    }

    let pw = passwords::PasswordGenerator {
        length: 28,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: true,
        spaces: false,
        exclude_similar_characters: false,
        strict: false,
    };
    let new_user = Users::ActiveModel {
        id: Set(Uuid::now_v7()),
        is_admin: Set(false),
        active: Set(true),
        username: Set(payload.username.to_string()),
        password: Set(pw.generate_one().unwrap().to_string()),
    };
    // let conn = pool.get().await.map_err(internal_error)?;
    let new_user: Users::Model = new_user.insert(&pool).await.expect("err");

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    Ok((StatusCode::CREATED, Json(new_user)))
}


