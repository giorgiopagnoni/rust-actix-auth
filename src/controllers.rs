extern crate validator;

use actix_web::{web, Error};
use serde_derive::Deserialize;
use validator::{Validate, ValidationErrors};
use actix_web::HttpResponse;
use futures::IntoFuture;
use crate::models::*;
use crate::schema::auth_users::dsl::*;
use diesel::prelude::*;

#[derive(Debug, Validate, Deserialize)]
pub struct RegisterUserRequest {
    #[validate(email())]
    email: String,
    #[validate(length(min = 8))]
    password: String,
}

pub fn user_register(
    usr_req: web::Json<RegisterUserRequest>,
    db_pool: web::Data<Pool>)
    -> impl IntoFuture<Item=HttpResponse, Error=Error> {

    let res: Result<(), ValidationErrors> = usr_req.validate();
    if res.is_err() {
        // TODO send back validation error messages
        return HttpResponse::BadRequest().finish();
    }

    let conn = db_pool.get().unwrap();
    let results: Vec<User> = auth_users
        .order(id.asc())
        .limit(1)
        .load::<User>(&conn)
        .expect("Error loading posts");

    println!("{:?}", results);

    // TODO check unique email

    // TODO store in db

    // TODO send registration email

    HttpResponse::Ok().finish()
}