use actix_web::web;

use crate::infrastructure::controllers::{create_person_controller::create_person_controller};

pub fn person_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/person")
            .route(web::post().to(create_person_controller))
    );
}