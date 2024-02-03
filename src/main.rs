use actix_web::{web, App, HttpRequest, HttpServer};

mod controllers;
mod config;
mod services;
mod models;
mod constants;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // env_logger::init_from_env(env_logger::Env::new().default().filter_or("info"));
    log::info!("Starting HTTP Server at port :8080");

    HttpServer::new(|| {
        App::new()
        .service(web::resource("/").to(hello::hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use actix_web::{body::to_bytes, dev::Service, http, test, web, App, Error};
    use super::*;

    #[actix_web::test]
    async fn test_index() -> Result<(), Error> {
        let app = App::new().route("/", web::get().to(hello));
        let app = test::init_service(app).await;

        let request = test::TestRequest::get().uri("/").to_request();
        let response = app.call(request).await?;

        assert_eq!(response.status(), http::StatusCode::OK);

        let response_body = response.into_body();
        assert_eq!(to_bytes(response_body).await?, r##"Hello, World"##);

        Ok(())
    }
}