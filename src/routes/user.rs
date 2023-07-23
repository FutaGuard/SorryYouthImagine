use axum::http::StatusCode;
use axum::{Json, extract::State};
use uuid::Uuid;
use sorry_youth_imagine::models::Users;


async fn create_user(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Json(payload): Json<Users>,
) -> (StatusCode, Json<Users>) {
    // insert your application logic here
    // Uuid::new_v4();
    // let my_uuid = uuid::new_v4();
    // uuid::Uuid::new_v4();
    // Uuid::new_
    let new_uuid = Uuid::now_v7();
    let user = Users {
        id: new_uuid,
        is_admin: false,
        active: false,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}


