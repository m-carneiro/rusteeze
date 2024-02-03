use actix_web::{http::header::HeaderValue, web};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    config::db::Pool,
};

#[derive(Serialize, Deserialize)]
pub struct TokenBodyResponse {
    pub token: String,
    pub token_type: String,
}

pub fn signup(user: UserDTO, pool: &web::Data<Pool>) -> Result<String, ServiceError> {

}