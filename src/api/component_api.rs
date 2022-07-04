use actix_multipart::Multipart;
use actix_web::{
    get, post, put,
    web::{Data, Form, Json, Path},
    Error, HttpResponse,
};
use futures_util::stream::StreamExt as _;

use crate::{models::component_model::Component, repository::dynamodb_repo::DynamoRepo};

#[post("/component")]
pub async fn create_component(
    db: Data<DynamoRepo>,
    new_component: Form<Component>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    println!("ddsdsdsd");
    db.create_component(new_component).await;

    // iterate over multipart stream
    while let Some(item) = payload.next().await {
        let mut field = item?;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            println!("-- CHUNK: \n{:?}", std::str::from_utf8(&chunk?));
        }
    }

    Ok(HttpResponse::Ok().into())
}

#[get("/component/{id}")]
pub async fn get_component(db: Data<DynamoRepo>, path: Path<String>) -> HttpResponse {
    //get_user code goes here
    todo!()
}

#[put("/component/{id}")]
pub async fn update_component(
    db: Data<DynamoRepo>,
    path: Path<String>,
    new_component: Json<Component>,
) -> HttpResponse {
    //update_user code goes here
    todo!()
}
