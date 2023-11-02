use std::env;
use std::net::SocketAddr;
use axum::{response::Html, routing::get, routing::post, Router};

use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection};
use migration::{Migrator, MigratorTrait};


mod routes;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = Database::connect(database_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    let app = Router::new()
        .route("/", get(routes::ping::ping))
        .route("/root", get(root))
        .route("/api/users/create", post(routes::user::create_user))
        .with_state(conn);

    // run it
    let bind: &SocketAddr = &"0.0.0.0:3000".parse().unwrap();
    println!("listening on http://{}", bind);

    axum::Server::bind(bind)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}