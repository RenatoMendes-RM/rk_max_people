use actix_web::Responder;
use actix_web::HttpResponse;
use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use crate::container::container::CONTAINER;

use crate::application::use_cases::create_person_handler::CreatePersonCommand;

#[derive(Deserialize)]
pub struct CreatePersonDTO {
    name: String,
    document_type: String,
    document_number: String,
}
#[derive(Serialize)]
pub struct CreatePersonResponse {
    id: String,
    message: String,
}


pub async fn create_person_controller(request_body: Json<CreatePersonDTO>) -> impl Responder {

    let command: CreatePersonCommand = CreatePersonCommand {
        name: String::from(&request_body.name),
        document_type: String::from(&request_body.document_type),
        document_number: String::from(&request_body.document_number),
    };

    match CONTAINER.create_person_handler().execute(command).await {
        Ok(id) => {
            let response = CreatePersonResponse {
                id: id.clone(),
                message: format!("Person {} created", id),
            };
            HttpResponse::Ok().json(response)
        } Err(_) => {
            HttpResponse::NotFound().body("Something went wrong")
        }
    }
}