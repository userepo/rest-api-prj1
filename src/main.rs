use std::{env, time::Duration};

//use log::{debug, error, info, trace, warn};
use pretty_env_logger;

use std::sync::Arc;

use axum::{
    routing::{delete, get, post},
    Router,
};

use dotenvy::dotenv;

use persistance::{
    answers_dao::{AnswersDao, AnswersDaoImpl},
    questions_dao::{QuestionsDao, QuestionsDaoImpl},
};
use sqlx::postgres::PgPoolOptions;

mod handlers;
mod models;
mod persistance;

use handlers::*;

#[derive(Clone)]
pub struct AppState {
    pub questions_dao: Arc<dyn QuestionsDao + Send + Sync>,
    pub answers_dao: Arc<dyn AnswersDao + Send + Sync>,
}

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");

    pretty_env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(15))
        .connect(database_url.as_str())
	.await
        .expect("Failed to create Postgres connection pool!");

    let questions_dao = QuestionsDaoImpl::new(pool.clone());
    let answers_dao = AnswersDaoImpl::new(pool);

    let app_state = AppState {
    	questions_dao: Arc::new(questions_dao), 
    	answers_dao: Arc::new(answers_dao) 
    };

    let app = Router::new()
        .route("/questions", post(create_question).get(read_questions))
        .route("/questions/{id}", delete(delete_question))
        .route("/answers", post(create_answer))
        .route("/answers/{id}", get(read_answers).delete(delete_answer))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
