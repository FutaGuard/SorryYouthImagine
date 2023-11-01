pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection,
};
use dotenvy::dotenv;
use std::env;
use bb8::Pool;

pub async  fn establish_connection() -> Pool<AsyncDieselConnectionManager<AsyncPgConnection>> {
    dotenv().ok();
    type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    return Pool::builder().build(config).await.unwrap();
}
