use actix_web::web::{Form, Json};
use aws_sdk_dynamodb::{client::fluent_builders::PutItem, model::AttributeValue, Client, Error};
use uuid::Uuid;

use crate::models::component_model::Component;

const TABLE_NAME: &str = "components";

#[derive(Clone)]
pub struct DynamoRepo {
    pub client: Client,
}

impl DynamoRepo {
    pub async fn new() -> Self {
        let config = aws_config::load_from_env().await;
        let client = Client::new(&config);
        Self { client: client }
    }

    pub async fn create_component(&self, component: Form<Component>) -> Result<(), Error> {
        let uuid = Uuid::new_v4().to_string();
        let request = self
            .client
            .put_item()
            .table_name(TABLE_NAME.to_string())
            .item("id", AttributeValue::S(String::from(uuid)))
            .item("name", AttributeValue::S("button".to_string()));
        println!("{:?}", component.label);
        request.send().await.expect("unable to save data");
        Ok(())
    }

    fn get_component() {}
}
