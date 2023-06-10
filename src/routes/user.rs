use actix_web::{HttpResponse,post,web::{self,Json}, Responder};
use chrono::Utc;
use mongodb::{bson::doc,bson::oid::ObjectId};
use serde::{Deserialize, Serialize};
use crate::models::user::User;
use::mongodb::Client;
use bcrypt::{hash,verify};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};



#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}


#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email:String,
    pub fullname: String,
    pub password: String,
}

#[post("/register")]
async fn register(db: web::Data<Client>, req: Json<RegisterRequest>) -> impl Responder {
    let col = db.database("rustecom").collection("users");

    //check if email is valid
    match col.find_one(doc! {"email":&req.email}, None).await.unwrap() {
        Some(_) => return HttpResponse::Conflict().body("Email already exist!"),
        _ => (),
    }
    //  if let Some(_) = user.find_one(doc! {"email":&req.email}, None).await.unwrap(){
    //     return HttpResponse::Conflict().body("Email already exist!")
    // }
    
    // Hash the password using bcrypt
    let hashpassword = match hash(&req.password, bcrypt::DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(_) => return  HttpResponse::InternalServerError().finish(),
    };

    // Create a new user document
    let new_user = User {
        _id:Some(ObjectId::new()),
        fullname:req.fullname.clone(),
        email:req.email.clone(),
        password:hashpassword,
        created_at:bson::DateTime::now().into(),
        updated_at:bson::DateTime::now().into(),
    };

    let result = col.insert_one(new_user,None).await;

      match result{
          Ok(_) => HttpResponse::Ok().body("register successfully"),
        Err(err) => {
            println!("Error while getting, {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }

    // let hashed_password = hash(&user.password, 10);

    // let data = doc! {
    //     // "fullname":user.fullname.to_owned(),
    //     "email":user.email.to_owned(),
    //     "password":hashed_password,
    // };

    // let result = collection.insert_one(data, None).await;
    // match result{
    //       Ok(_) => HttpResponse::Ok().body("register successfully"),
    //     Err(err) => {
    //         println!("Error while getting, {:?}", err);
    //         HttpResponse::InternalServerError().finish()
    //     }
    // }
}