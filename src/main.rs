use std::env;
use std::net::SocketAddr;
use std::net::TcpListener;
use std::sync::Arc;
use axum::{response::Html, routing::get, routing::post, Router};

use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection};
use migration::{Migrator, MigratorTrait};
use axum_login::{
    axum_sessions::{async_session::MemoryStore, SessionLayer},
    secrecy::SecretVec,
    AuthLayer, AuthUser, RequireAuthorizationLayer, PostgresStore,
};
use ::entity::{users as Users};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use once_cell::sync::Lazy;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use tokio::signal;

mod routes;

pub struct AppState {
    db: DatabaseConnection,
}

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = Database::connect(database_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    // Login session layer

    // logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "sorry_youth_imagine=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let app = Router::new()
        .route("/", get(routes::ping::ping))
        .route("/root", get(root))
        .route("/api/users/create", post(routes::user::create_user))
        .with_state(Arc::new(
            AppState{
                db: conn,
            }
        ));

    // run it
    let bind = TcpListener::bind("localhost:3000").unwrap();
    tracing::info!("listening on http://{}", bind.local_addr().unwrap());

    axum::Server::from_tcp(bind).unwrap()
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
        let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

async fn root() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}