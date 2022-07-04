mod api;
mod models;
mod repository;

#[allow(dead_code)]
use dotenv::dotenv;
//modify imports below
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use api::component_api::{create_component, get_component, update_component};
use repository::dynamodb_repo::DynamoRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect(".env file not found");
    let db = DynamoRepo::new().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(create_component)
            .service(get_component)
            .service(update_component)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
