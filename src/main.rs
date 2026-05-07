
use std::{env, time::Duration};

use log::{debug, error, info, trace, warn};
use pretty_env_logger;

use axum::{
    routing::{delete, get, post},
    Router,
};

use dotenvy::dotenv;

use sqlx::postgres::PgPoolOptions;

mod handlers;
mod models;
mod persistance;

use handlers::*;
//use models::*;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");

    pretty_env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let _pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(15))
        .connect(database_url.as_str())
	.await
        .expect("Failed to create Postgres connection pool!");
    
    let app = Router::new()
        .route("/question", post(create_question))
        .route("/questions", get(read_questions))
        .route("/question", delete(delete_question))
        .route("/answer", post(create_answer))
        .route("/answers", get(read_answers))
        .route("/answer", delete(delete_answer));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
