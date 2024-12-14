use actix_web::{web, App, HttpResponse, HttpServer};

use actix_cors::Cors;

mod infrastructure;
mod application;
mod domain;
mod container;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            // .app_data(web::Data::new(container.clone()))
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173") // Substitua pela URL do seu frontend
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"]) // Métodos permitidos
                    .allowed_headers(vec!["Content-Type", "Authorization"]) // Cabeçalhos permitidos
            )
            .configure(infrastructure::routes::person::person_routes)
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("/") }),
            )
    })

        .bind(("127.0.0.1", 8072))?
        .run()
        .await
}