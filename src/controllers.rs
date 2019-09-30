use crate::dataservice::{DataService, DS};

use actix_web::{web, Error, HttpResponse};
use serde_derive::Deserialize;
use validator::{Validate, ValidationErrors};
use validator_derive::Validate;
use futures::Future;
use futures::future::ok as fut_ok;

use lettre_email::Email;
use lettre::{SmtpClient, Transport};
use std::borrow::Borrow;


#[derive(Debug, Validate, Deserialize)]
pub struct RegisterUserRequest {
    #[validate(email(message = "Invalid email"))]
    pub email: String,
    #[validate(length(min = 8, message = "Must be at least 8 char long"))]
    pub password: String,
}

pub fn user_register(usr_req: web::Json<RegisterUserRequest>,
                     ds: web::Data<DS>) -> impl Future<Item=HttpResponse, Error=Error> {

    // validate request
    let res: Result<(), ValidationErrors> = usr_req.validate();
    if res.is_err() {
        let errs = res.err().unwrap();
        let err_resp = serde_json::to_string(&errs).unwrap();
        return fut_ok(HttpResponse::BadRequest().body(err_resp));
    }

    // check email uniqueness
    if ds.is_email_taken(&usr_req.email) {
        return fut_ok(HttpResponse::Conflict().finish());
    }

    // store user
    let token = ds.persist_new_user(&usr_req);

    // send registration email (or let another process handle that?)
    let email = Email::builder()
        .from("giorgio@example.com")
        .to(usr_req.email.borrow())
        .subject("Verify your email")
        .text(format!("Click {}", &token))
        .build();

    if email.is_ok() {
        let mut mailer = SmtpClient::new_unencrypted_localhost().unwrap().transport();
        mailer.send(email.unwrap().into());
    }

    fut_ok(HttpResponse::Created().finish())
}

pub fn user_verify(token: web::Path<String>, ds: web::Data<DS>)
                   -> impl Future<Item=HttpResponse, Error=Error> {
    if ds.activate_by_token(&token) {
        return fut_ok(HttpResponse::Ok().finish());
    }

    return fut_ok(HttpResponse::NotFound().finish());
}
