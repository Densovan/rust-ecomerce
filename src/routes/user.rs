use actix_web::{HttpResponse,post,web::{self,Json}, Responder};
use chrono::{Utc,Duration};
use mongodb::{bson::doc,bson::oid::ObjectId, Collection};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::models::user::User;
use::mongodb::Client;
use bcrypt::{hash,verify};
use jsonwebtoken::{ encode,  EncodingKey, Header};




#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email:String,
    pub fullname: String,
    pub password: String,
}

    #[derive(Serialize)]
    pub struct RegsiterRespon{
        pub status:String,
        pub msg:String
    }

#[post("/register")]
async fn register(db: web::Data<Client>, req: Json<RegisterRequest>) -> impl Responder {
    let col = db.database("rustecom").collection("users");

    //check if email is valid
    match col.find_one(doc! {"email":&req.email}, None).await.unwrap() {
        Some(_) => return HttpResponse::BadRequest().json(&RegsiterRespon{
             status:"400".to_string(),
            msg:"Email already exist!".to_string(),
        }),
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


let message_response = RegsiterRespon {
    status:"200".to_string(),
    msg:"Register successfully".to_string(),
};
      match result{
          Ok(_) => {
            HttpResponse::Ok().json(message_response)
        }
        Err(err) => {
            println!("Error while getting, {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}


#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email:String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Option<ObjectId>,
    pub fullname: String,
    pub exp: usize,
}

#[post("/login")]
async fn login (db: web::Data<Client>, req: Json<LoginRequest>) -> impl Responder {
       let col:Collection<User> = db.database("rustecom").collection("users");
    let data = &LoginRequest{
        email: req.email.clone(),
        password:req.password.clone(),
    };
    // check if email valid
    if let Ok(Some(user)) = col.find_one(doc! {"email":&data.email.clone()}, None).await {
        // let hashed_password = user.get_str("password").unwrap();
        //verify password
        if let Ok(valid) = verify(&req.password, &user.password){
            if valid {
                //Generate JWT Token
                let token = encode(&Header::default(), &Claims {
                    sub:user._id.clone(),
                    fullname:user.fullname.clone(),
                    exp:(Utc::now() + Duration::hours(24)).timestamp() as usize,
                }, &EncodingKey::from_secret("JWT_SECRET".as_ref()),).unwrap();
               return HttpResponse::Ok().json(json!({ "token": token }));
            }
        }
    }
    HttpResponse::Unauthorized().finish()
}