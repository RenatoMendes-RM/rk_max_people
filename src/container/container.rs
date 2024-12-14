use std::sync::Arc;
use crate::infrastructure::repository::person_repository::{PostgresRepository, Repository};

use crate::application::use_cases::{
    create_person_handler::CreatePersonHandler
};
use once_cell::sync::Lazy;


pub enum RepositoryType {
    Postgres,
}


#[derive(Clone)]
pub struct Container<'a> {
    create_person_handler: CreatePersonHandler<'a>
}

impl<'a> Container<'a> {
    pub fn new(repository_type: RepositoryType) -> Self {
        let repository: Arc<dyn Repository + Send + Sync> = match repository_type {
            RepositoryType::Postgres => Arc::new(PostgresRepository {}),
        };

        let create_person_handler = CreatePersonHandler::new(repository.clone());

        Container {
            create_person_handler
        }
    }

    pub fn create_person_handler(&self) -> &CreatePersonHandler<'a> {
        &self.create_person_handler
    }

}

pub static CONTAINER: Lazy<Container<'static>> = Lazy::new(|| {
    let container = Container::new(RepositoryType::Postgres);
    container
});