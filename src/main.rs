use std::env;
use std::time::Duration;
use log::{debug, error, info, trace, warn};
use pretty_env_logger;
use dotenvy;
use sqlx::postgres::PgPoolOptions;

use axum::{
    routing::{delete, get, post},
    Router,
};

mod handlers;
mod models;

use handlers::*;
use models::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect(".env file not found");

    pretty_env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Examples: https://github.com/launchbadge/sqlx
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(30))
        .connect(database_url.as_str()).await.unwrap(); //panic on error

    // This is just some test code to make sure we can connect to the database.
    let recs = sqlx::query_as::<_, QuestionDetail>("SELECT * FROM questions")
        .fetch_all(&pool)
        .await.unwrap(); // panic on error

    info!("------ Question Records ------");
    info!("{:?}", recs);
    
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
