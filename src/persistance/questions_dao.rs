use crate::models::{DBError, Question, QuestionDetail};
use async_trait::async_trait;
use futures_util::TryStreamExt;
use log::warn;
use sqlx::types::Uuid;
use sqlx::types::time::{OffsetDateTime};
use sqlx::{PgPool, Row};

#[async_trait]
pub trait QuestionsDao {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError>;
    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError>;
    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError>;
}

pub struct QuestionsDaoImpl {
    db: PgPool,
}

impl QuestionsDaoImpl {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

fn to_db_err_other(e: sqlx::Error) -> DBError {
    DBError::Other(Box::new(e))
}

#[async_trait]
impl QuestionsDao for QuestionsDaoImpl {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError> {
        let record = sqlx::query(
            r#"
            INSERT INTO questions ( title, description )
            VALUES ($1, $2)
            RETURNING
                question_uuid,
                title,
                description,
                created_at
            "#,
        )
        .bind(question.title)
        .bind(question.description)
        .fetch_one(&self.db)
        .await
        .map_err(|e| DBError::Other(Box::new(e)))?;

        let question_uuid: Uuid = record.try_get("question_uuid").map_err(to_db_err_other)?;
        let created_at: OffsetDateTime = record.try_get("created_at").map_err(to_db_err_other)?;

        Ok(QuestionDetail {
            question_uuid: question_uuid.to_string(),
            title: record.try_get("title").map_err(to_db_err_other)?,
            description: record.try_get("description").map_err(to_db_err_other)?,
            created_at: created_at.to_string(),
        })
    }

    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError> {
        let uuid = Uuid::try_parse(question_uuid.as_str())
            .map_err(|_| DBError::InvalidUUID(question_uuid))?;

        let result = sqlx::query("DELETE FROM questions WHERE question_uuid = $1")
            .bind(uuid)
            .execute(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        if result.rows_affected() == 0 {
            warn!("Question not deleted - not found in the DB");
        }

        Ok(())
    }

    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError> {
        let mut records = sqlx::query("SELECT question_uuid, title, description, created_at FROM questions")
            .fetch(&self.db);

        let mut questions: Vec<QuestionDetail> = Vec::new();

        while let Some(record) = records.try_next().await.map_err(to_db_err_other)? {
            let question_uuid: Uuid = record.try_get("question_uuid").map_err(to_db_err_other)?;
            let created_at: OffsetDateTime = record.try_get("created_at").map_err(to_db_err_other)?;

            let question_detail = QuestionDetail {
                question_uuid: question_uuid.to_string(),
                title: record.try_get("title").map_err(to_db_err_other)?,
                description: record.try_get("description").map_err(to_db_err_other)?,
                created_at: created_at.to_string(),
            };
            questions.push(question_detail);
        }

        Ok(questions)
    }
}
