use std::sync::Arc;
use crate::domain::entity::person::Person;

use serde::Deserialize;
use crate::infrastructure::repository::person_repository::{Repository, RepositoryError}; // Importa o tipo de erro correto

use async_trait::async_trait;
use tokio::sync::Mutex;



#[derive(Deserialize, Clone)]  // Adicione Clone aqui
pub struct CreatePersonCommand {
    pub name: String,
    pub document_type: String,
    pub document_number: String
}

#[derive(Clone)]
pub struct CreatePersonHandler<'a> {
    repository: Arc<dyn Repository + Sync + Send>,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> CreatePersonHandler<'a> {
    pub fn new(repository: Arc<dyn Repository + Sync + Send>) -> Self {
        CreatePersonHandler {
            repository,
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn execute(&self, command: CreatePersonCommand) -> Result<String, ()> {


        let person = Person::new(&command.name, &command.document_type, &command.document_number);

        let cloned_person = person.clone();

        let repository = Arc::clone(&self.repository);
        tokio::spawn(async move {
            repository.save(&cloned_person).await.unwrap();
        });

        Ok(person.id)
    }
}


// Mock do repositório
struct MockRepository {
    pub saved_persons: Arc<Mutex<Vec<Person>>>,
}

#[async_trait]
impl Repository for MockRepository {
    async fn save(&self, person: &Person) -> Result<(), RepositoryError> { // Retorna o tipo correto de erro
        let mut persons = self.saved_persons.lock().await;
        persons.push(person.clone());
        Ok(())
    }
}

#[tokio::test]
async fn test_create_person_success() {
    // Arrange
    let mock_repo = Arc::new(MockRepository {
        saved_persons: Arc::new(Mutex::new(vec![])),
    });
    let handler = CreatePersonHandler::new(mock_repo.clone());
    let command = CreatePersonCommand {
        name: "John Smith".to_string(),
        document_type: "1".to_string(),
        document_number: "0123456789".to_string(),
    };

    // Act
    let result = handler.execute(command.clone()).await;

    // Assert
    assert!(result.is_ok());
    let person_id = result.unwrap();
    assert!(!person_id.is_empty());

    // Verifica se o usuário foi salvo no repositório
    let saved_persons = mock_repo.saved_persons.lock().await;
    assert_eq!(saved_persons.len(), 1); // Correção aqui
    assert_eq!(saved_persons[0].name, command.name);
}
