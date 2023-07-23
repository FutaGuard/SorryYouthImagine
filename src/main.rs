use std::env;
use std::net::SocketAddr;
use axum::{response::Html, routing::get, Router};

mod routes;


#[tokio::main]
async fn main() {
    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = deadpool_diesel::postgres::Manager::new(database_url, deadpool_diesel::Runtime::Tokio1);
    let pool = deadpool_diesel::postgres::Pool::builder(manager)
        .build()
        .unwrap();

    let app = Router::new()
        .route("/", get(routes::ping::ping))
        .route("/root", get(root))
        .with_state(pool);

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