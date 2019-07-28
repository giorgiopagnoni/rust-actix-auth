use dotenv;
use actix_web::{HttpServer, App, web, HttpResponse, guard, middleware};
use rust_auth::controllers::user_register;
use rust_auth::dataservice::{MysqlR2D2DataService};
use actix_identity::{IdentityService, CookieIdentityPolicy};

fn main() {
    dotenv::dotenv().ok();

    // domain
    let domain = std::env::var("DOMAIN").unwrap_or("localhost".to_string());
    // socket
    let http_addr = std::env::var("HTTP_ADDR").expect("HTTP_ADDR must be set");
    // secret
    let secret = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");
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
                    .route("/verify/{token}", web::get().to(|| HttpResponse::NotFound()))
                    .route("/login", web::post().to(|| HttpResponse::NotFound()))
            )
    })
        .bind(http_addr)
        .unwrap()
        .run()
        .unwrap()
}