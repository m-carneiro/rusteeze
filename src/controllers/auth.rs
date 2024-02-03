use actix_web::{web, HttpResponse, HttpServer};

use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::{
        response::ResponseBody,
        user::{LoginDTO, UserDTO},
    },
    services::auth_service,
};

pub async fn signup(
    user_dto: web::Json<UserDto>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    match auth_service::signup(user_dto.0, &pool) {
        Ok(message) => Ok(HttpResponse::Ok().json(ResponseBody::new(
            true,
            StatusCode::OK,
            message,
        ))),
        Err(err) => Ok(HttpResponse::BadRequest().json(ResponseBody::new(
            false,
            StatusCode::BAD_REQUEST,
            err.to_string(),
        ))),
    }
}