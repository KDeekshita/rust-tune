use serde::{Deserialize, Serialize};


#[derive(Deserialize , Serialize)]
pub struct  SignupBody {
   pub  username : String,
   pub  password : String
}

pub enum AuthResult {
   Valid,
   UserNotFound,
   WrongPassword,
}