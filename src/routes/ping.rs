use axum::response::Html;
use chrono::prelude::Utc;



pub async fn ping() -> Html<String> {
    // let text = /**/;
    return Html(format!("pong, {}", Utc::now().to_string()))
}