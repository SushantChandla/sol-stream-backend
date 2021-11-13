#[macro_use]
extern crate diesel;

mod models;
mod routes;
mod schema;
mod solana;
use rocket::routes;
use solana::get_all_program_accounts;
use solana::subscribe_to_program;

use crate::models::Stream;
use crate::routes::get_all_stream;
use crate::routes::index;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program_accounts = get_all_program_accounts();
    let conn = establish_connection();
    for item in program_accounts.iter() {
        let stream = Stream::new(item.0.to_string(), &item.1.data);
        match stream {
            Some(a) => Stream::insert_or_update(a, &conn),
            _ => continue,
        };
    }

    subscribe_to_program();

    let cors = rocket_cors::CorsOptions::default().to_cors()?;

    rocket::build()
        .mount("/", routes![index, get_all_stream])
        .attach(cors)
        .launch()
        .await?;

    Ok(())
}
