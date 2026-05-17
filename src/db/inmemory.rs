use std::io::Error;

use crate::types::auth::AuthResult;



 struct User {
    id : String,
    username : String,
    password : String
}



pub struct Db {
     conn : Vec<User>
}

impl Db {


    pub fn new()-> Self {
        return  Db {
            conn : Vec::new()
        }
    }

    pub fn create(&mut self , username : &String , password : &String , id : &String ) -> Result<bool, Error >{

        let user_exist = self.conn.iter().position(|x| *x.username == *username);

        match user_exist {
            Some(_pos) => {
                return Ok(false);
            }

            None => {

                let user = User {
                    id : id.to_string(), 
                    username : username.to_string() , 
                    password : password.to_string()
                };


                self.conn.push(user);

                return Ok(true)



            }
        }
    }

    pub fn update(&mut self , username : String , password : String ) -> Result<bool, Error> {
        
        for i in &mut self.conn {
            if i.username == username  {
                i.password = password;

                return Ok(true);
            }
        }
        return Ok(false);
    }



    pub fn delete(&mut self , username : String  ) -> Result<bool , Error> {

       let user_exist = self.conn.iter().position(|x| *x.username == username);

       match user_exist {
        Some(pos) => {
            self.conn.remove(pos);
            return Ok(true);
        },
        None => {
            return Ok(false);
        }
       }

    }

    pub fn is_valid(&self , username : &String  , password : &String ) -> Result<AuthResult , Error> {

        let user_exist = self.conn.iter().find(|x| *x.username == *username);

        match user_exist {
            Some(user) => {
                
                if user.password == *password {
                    return Ok(AuthResult::Valid);
                }else {
                    return Ok(AuthResult::WrongPassword); 
                }
            }

            None => {
                return Ok(AuthResult::UserNotFound); 
            }
        }
        
    }

} 