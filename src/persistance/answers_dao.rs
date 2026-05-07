use crate::models::{Answer, AnswerDetail, DBError, postgres_error_codes};
use async_trait::async_trait;
use log::{info, warn};
use sqlx::types::Uuid;
use sqlx::types::time::{OffsetDateTime};
use sqlx::{PgPool, Row};

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
        Self { db }
    }
}

fn to_db_err_other(e: sqlx::Error) -> DBError {
    DBError::Other(Box::new(e))
}

#[async_trait]
impl AnswersDao for AnswersDaoImpl {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError> {
        let question_uuid = answer.question_uuid.clone();

        let uuid = Uuid::try_parse(answer.question_uuid.as_str())
            .map_err(|_| DBError::InvalidUUID(answer.question_uuid))?;

        let record = sqlx::query(
            r#"
        INSERT INTO answers ( question_uuid, content )
        VALUES ( $1, $2 )
        RETURNING
            answer_uuid,
            question_uuid,
            content,
            created_at
                "#,
        )
        .bind(uuid) // $1
        .bind(answer.content)
        .fetch_one(&self.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err)
                if db_err.code().as_deref() == Some(postgres_error_codes::FOREIGN_KEY_VIOLATION) =>
            {
                DBError::InvalidUUID(question_uuid)
            }
            _ => DBError::Other(Box::new(e)),
        })?;

        let answer_uuid: Uuid = record.try_get("answer_uuid").map_err(to_db_err_other)?;
        let question_uuid: Uuid = record.try_get("question_uuid").map_err(to_db_err_other)?;
        let created_at: OffsetDateTime  =
            record.try_get("created_at").map_err(to_db_err_other)?;

        Ok(AnswerDetail {
            answer_uuid: answer_uuid.to_string(),
            question_uuid: question_uuid.to_string(),
            content: record.try_get("content").map_err(to_db_err_other)?,
            created_at: created_at.to_string(),
        })
    }

    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError> {
        let uuid =
            Uuid::try_parse(answer_uuid.as_str()).map_err(|_| DBError::InvalidUUID(answer_uuid))?;

        let result = sqlx::query("DELETE FROM answers WHERE answer_uuid = $1")
            .bind(uuid)
            .execute(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        if result.rows_affected() == 0 {
            warn!("Answer not deleted - not found in the DB");
        }

        Ok(())
    }

    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError> {
        let uuid = Uuid::try_parse(question_uuid.as_str())
            .map_err(|_| DBError::InvalidUUID(question_uuid))?;

        let records = sqlx::query("SELECT answer_uuid, question_uuid, content, created_at FROM answers WHERE question_uuid = $1")
            .bind(uuid)
            .fetch_all(&self.db)
            .await
            .map_err(to_db_err_other)?;

        let answers = records
            .into_iter()
            .map(|record| -> Result<AnswerDetail, DBError> {
                let answer_uuid: Uuid = record.try_get("answer_uuid").map_err(to_db_err_other)?;
                let question_uuid: Uuid = record.try_get("question_uuid").map_err(to_db_err_other)?;
                let created_at: OffsetDateTime  = record.try_get("created_at").map_err(to_db_err_other)?;

                Ok(AnswerDetail {
                    answer_uuid: answer_uuid.to_string(),
                    question_uuid: question_uuid.to_string(),
                    content: record.try_get("content").map_err(to_db_err_other)?,
                    created_at: created_at.to_string(),
                })
            })
            .collect::<Result<Vec<AnswerDetail>, DBError>>()?;

        Ok(answers)
    }
}
