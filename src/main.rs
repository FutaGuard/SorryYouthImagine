use std::net::SocketAddr;
use axum::{response::Html, routing::get, Router};

mod routes;

// routes::hello;


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(routes::ping::ping))
        .route("/root", get(root));

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