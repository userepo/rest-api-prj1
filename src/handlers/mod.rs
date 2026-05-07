use axum::{extract::State, response::IntoResponse, Json};

use crate::{models::*, AppState};

mod handlers_inner;

// ---- Questions ----

pub async fn create_question(
    State(AppState { questions_dao, .. }): State<AppState>,
    Json(question): Json<Question>
) -> impl IntoResponse {
    Json(QuestionDetail {
        question_uuid: "question_uuid".to_owned(),
        title: "title".to_owned(),
        description: "description".to_owned(),
        created_at: "created_at".to_owned(),
    })
}

pub async fn read_questions(
    State(AppState { questions_dao, .. }): State<AppState>
) -> impl IntoResponse {
    Json(vec![QuestionDetail {
        question_uuid: "question_uuid".to_owned(),
        title: "title".to_owned(),
        description: "description".to_owned(),
        created_at: "created_at".to_owned(),
    }])
}

pub async fn delete_question(
    State(AppState { questions_dao, .. }): State<AppState>,
    Json(question_uuid): Json<QuestionId>,
) {
    // ...
}

// ---- Answers ----

pub async fn create_answer(
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(answer): Json<Answer>,
) -> impl IntoResponse {
    Json(AnswerDetail {
        answer_uuid: "answer_uuid".to_owned(),
        question_uuid: "question_uuid".to_owned(),
        content: "content".to_owned(),
        created_at: "created_at".to_owned(),
    })
}

pub async fn read_answers(
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(question_uuid): Json<QuestionId>,
) -> impl IntoResponse {
    Json(vec![AnswerDetail {
        answer_uuid: "answer_uuid".to_owned(),
        question_uuid: "question_uuid".to_owned(),
        content: "content".to_owned(),
        created_at: "created_at".to_owned(),
    }])
}

pub async fn delete_answer(
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(answer_uuid): Json<AnswerId>,
) {
    // ...
}