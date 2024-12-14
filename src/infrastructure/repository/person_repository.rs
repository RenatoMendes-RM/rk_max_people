use std::sync::{Arc, Mutex};
use tokio_postgres::NoTls;
use async_trait::async_trait;

use crate::domain::entity::person::Person;

#[derive(Debug, Clone)]
pub enum RepositoryError {
    NotFound,
    DatabaseError(String),
}

impl std::fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepositoryError::NotFound => write!(f, "Person not found"),
            RepositoryError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

#[async_trait]
pub trait Repository: Send + Sync {
    async fn save(&self, person: &Person) -> Result<(), RepositoryError>;
}




#[derive(Clone, Default)]
pub struct InMemoryRepository {
    persons: Arc<Mutex<Vec<Person>>>,
}






#[async_trait::async_trait]
impl Repository for InMemoryRepository {
    async fn save(&self, person: &Person) -> Result<(), RepositoryError> {
        let mut persons = self.persons.lock().unwrap();
        persons.push(person.clone());
        Ok(())
    }

}




#[derive(Clone)]
pub struct PostgresRepository {}

impl PostgresRepository {
    async fn connect() -> tokio_postgres::Client {
        let db_url = "postgres://username:password@localhost:5435/rk_max_people_db";
        let (client, connection) = tokio_postgres::connect(db_url, NoTls)
            .await
            .expect("Error connect database");

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Error connect database: {}", e);
            }
        });
        client
    }

    async fn create_tables() {
        let client = Self::connect().await;
        client
            .execute(
                r#"CREATE TABLE IF NOT EXISTS persons (
                    "id" VARCHAR(255) PRIMARY KEY,
                    "name" VARCHAR(255) NOT NULL,
                    "document_type" VARCHAR(255) NOT NULL,
                    "document_number" VARCHAR(255) NOT NULL
                )"#,
                &[],
            )
            .await
            .expect("Error when create table persons");
    }
}


#[async_trait]
impl Repository for PostgresRepository {
    async fn save(&self, person: &Person) -> Result<(), RepositoryError> {
        Self::create_tables().await;
        let client = Self::connect().await;

        match client.execute(
            "INSERT INTO persons (id, name, document_type, document_number) VALUES ($1, $2, $3, $4)",
            &[&person.id, &person.name, &&person.document_type, &&person.document_number],
        ).await {
            Ok(_) => Ok(()),
            Err(e) => Err(RepositoryError::DatabaseError(e.to_string())),
        }
    }
}

