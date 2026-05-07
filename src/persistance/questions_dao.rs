use async_trait::async_trait;
use futures_util::TryStreamExt;
use sqlx::types::Uuid;
use sqlx::{PgPool};

use crate::models::{DBError, Question, QuestionDetail};

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
        QuestionsDaoImpl { db }
    }
}

fn to_db_err_other(e: sqlx::Error) -> DBError {
    DBError::Other(Box::new(e))
}

#[async_trait]
impl QuestionsDao for QuestionsDaoImpl {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError> {
        let record = sqlx::query!(
            r#"
            INSERT INTO questions ( title, description )
            VALUES ($1, $2)
            RETURNING
                question_uuid,
                title,
                description,
                created_at
            "#,
            question.title,
            question.description
        )
        .fetch_one(&self.db)
        .await
        .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(QuestionDetail {
            question_uuid: record.question_uuid.to_string(),
            title: record.title,
            description: record.description,
            created_at: record.created_at.to_string(),
        })
    }

    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError> {
        let uuid = Uuid::try_parse(question_uuid.as_str())
            .map_err(|_| DBError::InvalidUUID(question_uuid))?;

        sqlx::query!("DELETE FROM questions WHERE question_uuid = $1", uuid)
            .execute(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(())
    }

    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError> {
        let mut records =
            sqlx::query!("SELECT question_uuid, title, description, created_at FROM questions")
                .fetch(&self.db);

        let mut questions: Vec<QuestionDetail> = Vec::new();

        while let Some(record) = records.try_next().await.map_err(to_db_err_other)? {
            questions.push(QuestionDetail {
                question_uuid: record.question_uuid.to_string(),
                created_at:    record.created_at.to_string(),
                title:         record.title,
                description:   record.description,
            });
        }

        Ok(questions)
    }
}
