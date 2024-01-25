use actix_web::{middleware, web, App, HttpRequest, HttpServer};

async fn index(request: HttpRequest) -> &'static str {
    println!("REQ: {request:?}");
    "Hello, World"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // env_logger::init_from_env(env_logger::Env::new().default().filter_or("info"));
    log::info!("Starting HTTP Server at port :8080");

    HttpServer::new(|| {
        App::new()
        .service(web::resource("path").to(|| async { "Hello World!" }))
        .service(web::resource("path").to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

mod tests {
    use actix_web::{body::to_bytes, dev::Service, http, test, web, App, Error};
    use super::*;

    #[actix_web::test]
    async fn test_index() -> Result<(), Error> {
        let app = App::new().route("/", web::get().to(index));
        let app = test::init_service(app).await;

        let request = test::TestRequest::get().uri("/").to_request();
        let response = app.call(request).await?;

        assert_eq!(response.status(), http::StatusCode::OK);

        let response_body = response.into_body();
        assert_eq!(to_bytes(response_body).await?, r##"Hello, World"##);

        Ok(())
    }
}