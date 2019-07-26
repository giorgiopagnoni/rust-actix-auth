extern crate validator;
extern crate bcrypt;

use actix_web::{web, Error};
use serde_derive::Deserialize;
use validator::{Validate, ValidationErrors};
use actix_web::HttpResponse;
use futures::Future;
use futures::future::ok as fut_ok;
use crate::models::*;
use uuid::Uuid;
use bcrypt::hash;
use lettre_email::Email;
use lettre::{SmtpClient, Transport};
use std::borrow::Borrow;
use std::collections::HashMap;

#[derive(Debug, Validate, Deserialize)]
pub struct RegisterUserRequest {
    #[validate(email(message = "Invalid email"))]
    email: String,
    #[validate(length(min = 8))]
    password: String,
}

pub fn user_register(
    usr_req: web::Json<RegisterUserRequest>,
    db_pool: web::Data<Pool>)
    -> impl Future<Item=HttpResponse, Error=Error> {

    // validate request
    let res: Result<(), ValidationErrors> = usr_req.validate();
    if res.is_err() {
        // TODO?
        let mut err_map: HashMap<&str, Vec<_>> = HashMap::new();
        for (k, v) in &res.err().unwrap().field_errors() {
            err_map.insert(k, v.iter().map(|e| e.message.borrow()).collect());
        }
        return fut_ok(HttpResponse::BadRequest().finish());
    }

    // check email uniqueness
    let mut conn = db_pool.get().unwrap();
    if conn.prep_exec("SELECT * FROM auth_users WHERE email = :email LIMIT 1",
                      params! {"email" => &usr_req.email})
        .unwrap().next().is_some() {
        return fut_ok(HttpResponse::Conflict().finish());
    }

    // store user
    let token = Uuid::new_v4().to_string();
    conn.prep_exec(r"INSERT INTO auth_users (email, passwd, token)
                                    VALUES (:email, :passwd, :token)",
                   params! {
                        "email" => &usr_req.email,
                        "passwd" => hash(&usr_req.password, 6).unwrap(),
                        "token" => &token
                    })
        .unwrap();

    // send registration email (or let another process handle that?)
    let email = Email::builder()
        .from("giorgio@example.com")
        .to(usr_req.email.borrow())
        .subject("Verify your email")
        .text(format!("Click {}", &token))
        .build()
        .unwrap();

    let mut mailer = SmtpClient::new_unencrypted_localhost().unwrap().transport();
    mailer.send(email.into());

    return fut_ok(HttpResponse::Created().finish());
}