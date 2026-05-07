use async_trait::async_trait;
use sqlx::types::Uuid;
use sqlx::{PgPool};
use crate::models::{postgres_error_codes, Answer, AnswerDetail, DBError};

#[async_trait]
pub trait AnswersDao {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError>;
    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError>;
    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError>;
}

pub struct AnswersDaoImpl {
    db: PgPool,
}

impl AnswersDaoImpl {
    pub fn new(db: PgPool) -> Self {
        AnswersDaoImpl { db }
    }
}

#[async_trait]
impl AnswersDao for AnswersDaoImpl {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError> {
        let question_uuid = answer.question_uuid.clone();

        let uuid = Uuid::try_parse(answer.question_uuid.as_str())
            .map_err(|_| DBError::InvalidUUID(answer.question_uuid))?;

        let record = sqlx::query!(
            r#"
        INSERT INTO answers ( question_uuid, content )
        VALUES ( $1, $2 )
        RETURNING
            answer_uuid,
            question_uuid,
            content,
            created_at
                "#,
            uuid,
            answer.content
        )
        .fetch_one(&self.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err)
                if db_err.code().as_deref()
                    == Some(postgres_error_codes::FOREIGN_KEY_VIOLATION) =>
            {
                DBError::InvalidUUID(question_uuid)
            }
            _ => DBError::Other(Box::new(e)),
        })?;

        Ok(AnswerDetail {
            answer_uuid: record.answer_uuid.to_string(),
            question_uuid: record.question_uuid.to_string(),
            content: record.content,
            created_at: record.created_at.to_string(),
        })
    }

    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError> {
        let uuid =
            Uuid::try_parse(answer_uuid.as_str()).map_err(|_| DBError::InvalidUUID(answer_uuid))?;

        sqlx::query!("DELETE FROM answers WHERE answer_uuid = $1", uuid)
            .execute(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(())
    }

    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError> {
        let uuid = Uuid::try_parse(question_uuid.as_str())
            .map_err(|_| DBError::InvalidUUID(question_uuid))?;

        let records = sqlx::query!(
            "SELECT answer_uuid, question_uuid, content, created_at FROM answers WHERE question_uuid = $1", uuid)
            .fetch_all(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        let answers = records
            .iter()
            .map(|r| AnswerDetail {
                answer_uuid: r.answer_uuid.to_string(),
                question_uuid: r.question_uuid.to_string(),
                content: r.content.clone(),
                created_at: r.created_at.to_string(),
            })
            .collect();

        Ok(answers)
    }
}
