use crate::models::*;
use axum::{
    http::StatusCode,
    response::IntoResponse, Json};


// ---- Questions ----

pub async fn create_question(Json(question): Json<Question>) -> impl IntoResponse {
    let question_detail = QuestionDetail {
        question_uuid: "8f3c2c4e-9b6a-4d7e-9c1e-2e4a1f7d9b32".to_string(),
        title: question.title,
        description: question.description,
        created_at: "2026-05-02 15:35:10.000".to_string(),
    };

    (StatusCode::CREATED, Json(question_detail))
}

pub async fn read_questions() -> impl IntoResponse {
    let question_detail1 = QuestionDetail {
        question_uuid: "8f3c2c4e-9b6a-4d7e-9c1e-2e4a1f7d9b32".to_string(),
        title: "Newly Created Question 1".to_string(),
        description: "My Description 1".to_string(),
        created_at: "2026-05-02 15:35:10.000".to_string(),
    };
    let question_detail2 = QuestionDetail {
        question_uuid: "d1a7e0f4-3c8b-4c2e-9f55-0b7e2c4a8f91".to_string(),
        title: "Newly Created Question 2".to_string(),
        description: "My Description 2".to_string(),
        created_at: "2026-05-02 15:40:15.000".to_string(),
    };

    let question_details = vec![question_detail1, question_detail2];

    (StatusCode::OK, Json(question_details))
}

pub async fn delete_question(Json(question_id): Json<QuestionId>) {
    //let question_uuid = question_id.question_uuid;
    ()
}

// ---- Answers ----

pub async fn create_answer(Json(answer): Json<Answer>) -> impl IntoResponse {
    let answer_detail = AnswerDetail {
        answer_uuid: "4b92e7d1-6f3a-4c0b-8c44-1e0f9a3d7b20".to_string(),
        question_uuid: answer.question_uuid,
        content: answer.content,
        created_at: "2026-05-02 18:40:50.700".to_string(),
    };

    (StatusCode::CREATED, Json(answer_detail))
}

pub async fn read_answers(Json(question_id): Json<QuestionId>) -> impl IntoResponse {

    let answer_detail1 = AnswerDetail {
        answer_uuid: "4b92e7d1-6f3a-4c0b-8c44-1e0f9a3d7b20".to_string(),
        question_uuid: question_id.question_uuid,
        content: "Test question 1".to_string(),
        created_at: "2026-05-02 18:50:29.000".to_string(),
    };

    let answer_detail2 = AnswerDetail {
        answer_uuid: "4b92e7d1-6f3a-4c0b-8c44-1e0f9a3d7b20".to_string(),
        question_uuid: "d1a7e0f4-3c8b-4c2e-9f55-0b7e2c4a8f91".to_string(),
        content: "Test question 2".to_string(),
        created_at: "2026-05-02 18:55:20.000".to_string(),
    };

    let answer_details = vec![answer_detail1, answer_detail2];

    (StatusCode::OK, Json(answer_details))
}

pub async fn delete_answer(Json(answer_id): Json<AnswerId>) {
    //let answer_uuid = answer_id.answer_uuid;
    ()
}