use dotenv;
use actix_web::{HttpServer, App, web, HttpResponse, guard, middleware};
use rust_auth::controllers::{user_register, user_verify};
use rust_auth::dataservice::{MysqlR2D2DataService};


fn main() {
    dotenv::dotenv().ok();

    // domain
    let _domain = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());
    // socket
    let http_addr = std::env::var("HTTP_ADDR").expect("HTTP_ADDR must be set");
    // secret
    let _secret = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    // data service
    let ds = MysqlR2D2DataService::new();

    HttpServer::new(move || {
        App::new()
            .data(ds.clone())
            .wrap(middleware::DefaultHeaders::new().header("content-type", "application/json; charset=utf-8"))
            .wrap(middleware::Compress::default())
            .service(
                web::scope("/user")
                    .guard(guard::Header("content-type", "application/json"))

                    .route("/register", web::post().to_async(user_register))
                    .route("/verify/{token}", web::get().to_async(user_verify))
                    .route("/login", web::post().to(HttpResponse::NotFound))
            )
    })
        .bind(http_addr)
        .unwrap()
        .run()
        .unwrap()
}
