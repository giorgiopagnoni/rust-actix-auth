extern crate serde_json;
extern crate futures;
extern crate r2d2;

use dotenv;
use actix_web::{HttpServer, App, web, HttpResponse, guard, middleware};
use rust_auth::controllers::{user_register};
use rust_auth::models::Pool;
use mysql::OptsBuilder;

fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    // socket
    let http_addr = std::env::var("HTTP_ADDR").expect("HTTP_ADDR must be set");

    // db setup
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let opt = mysql::Opts::from_url(&db_url).unwrap();
    let manager = r2d2_mysql::MysqlConnectionManager::new(OptsBuilder::from_opts(opt));
    let db_pool: Pool = r2d2::Pool::builder().build(manager).expect("Failed to create DB connection pool");

    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
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
        .bind(http_addr)?
        .run()
}