use axum::http::StatusCode;
use axum::{Json, extract::State};
// use diesel::RunQueryDsl;
use diesel::prelude::*;
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection, RunQueryDsl,
};
use uuid::Uuid;
// use diesel::sql_types::Uuid;
use sorry_youth_imagine::models::Users;
use sorry_youth_imagine::schema::users;
use serde::{Serialize, Deserialize};

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
    State(pool): State<Pool>,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<Users>), (StatusCode, String)> {
    // insert your application logic here
    let new_user = Users {
        id: Uuid::now_v7(),
        is_admin: false,
        active: false,
        token: "".to_string(),
    };
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(|conn| {
            diesel::insert_into(users::table)
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


