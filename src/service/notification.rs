use crate::{model::subscriber::Subscriber, repository::subscriber::SubscriberRepository};
use bambangshop::{compose_error_response, Result};
use rocket::http::Status;

pub struct NotificationService;

impl NotificationService {
    pub fn subscribe(product_type: &str, subscriber: Subscriber) -> Result<Subscriber> {
        let product_type_upper = product_type.to_uppercase();
        let product_type_str = product_type_upper.as_str();
        let subscriber_result = SubscriberRepository::add(product_type_str, subscriber);
        Ok(subscriber_result)
    }

    pub fn unsubscribe(product_type: &str, url: &str) -> Result<Subscriber> {
        let product_type_upper = product_type.to_uppercase();
        let product_type_str = product_type_upper.as_str();
        let result = SubscriberRepository::delete(product_type_str, url);
        if result.is_none() {
            return Err(compose_error_response(
                Status::NotFound,
                String::from("Subscriber not found"),
            ));
        }
        return Ok(result.unwrap());
    }
}
