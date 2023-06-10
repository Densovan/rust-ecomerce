use mongodb::{error::Error, options,Client};
use dotenv::dotenv;

pub async fn db_pool() -> Result<Client,Error> {
    dotenv().ok();
   
   let mongodb_url = std::env:: var("DATABASE_URL").unwrap();

   let db_address = format!("{}",mongodb_url);    

   let mut client_option = options::ClientOptions::parse(&db_address).await?;
    client_option.retry_writes = Some(false);

    let client = Client::with_options(client_option);

    match client {
        Ok(c) =>{
            println!("Connected to mongodb");
            Ok(c)
        }
        Err(e) => Err(e)
    }

}
