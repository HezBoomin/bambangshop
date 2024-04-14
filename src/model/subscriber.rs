use rocket::serde::{Deserialize, Serialize};
use rocket::log;
use rocket::serde::json::to_string;
use rocket::tokio;
use bambangshop::REQWEST_CLIENT;
use crate::model::notification::Notification;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Notification {
    pub product_tittle: String,
    pub product_type: String,
    pub product_url : String,
    pub subscriber_name: String,
    pub status: String,
}