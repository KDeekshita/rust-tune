use std::sync::Mutex;

use actix_web::{HttpResponse, Responder, post, web};
use uuid::Uuid;

use crate::{db::inmemory::Db, types::auth::{AuthResult, SignupBody}};




pub async fn signup(body : web::Json<SignupBody> , data : web::Data<Mutex<Db>>) -> impl Responder {


    let mut db = data.lock().unwrap();

    

      let username =  &body.username;
      let password = &body.password;

      let id = Uuid::new_v4();

      let user = db.create(username, password, &id.to_string());


      match user {
        Ok(true) => {
          return HttpResponse::Ok().body("signup successfully");
        },
        Ok(false) => {
          return HttpResponse::Conflict().body("username already exist")
        },
        Err(_) => {
          return HttpResponse::InternalServerError().body("Internal Server Error")
        }
      }

}


pub async fn signin(body : web::Json<SignupBody> , data : web::Data<Mutex<Db>>) -> impl Responder {


  let db = data.lock().unwrap();

    let username =  &body.username;
    let password = &body.password;

    let user = db.is_valid(username, password);

    match user {
      Ok(auth) => {
        match auth {
          AuthResult::Valid => {
            return HttpResponse::Ok().body("here is the jwt token")
          },
          AuthResult::UserNotFound => {
            return HttpResponse::NotFound().body("User Not Found")
          },
          AuthResult::WrongPassword => {
            return HttpResponse::Unauthorized().body("Password does not match")
          }
        }
      },
      Err(_) => {
        return HttpResponse::InternalServerError().body("Internal server Error");
      }
    }

}


