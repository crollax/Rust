
#[macro_use]
extern crate diesel;

// #[macro_use]
// extern crate diesel_codegen;
extern crate dotenv;

// use dotenv;

use std::env;

mod schema;
mod models;

fn main() {
    // check to see if dotenv file
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn = PgConnection::establish(&database_url).unwrap();

    let book = models::NewBook {
        title: String::from("Gravity's Rainbow"),
        author: String::from("Thomas Pynchon"),
        published: true,
    };

    if models::Book::insert(book, &conn) {
        println!("success");
    } else {
        println!("failed");
    }
}
