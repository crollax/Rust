#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
// #[macro_use]
extern crate rocket_contrib;
// #![macro_use]
extern crate mysql;

mod routes;
// mod examples;

// #[database("rocket_app")]
// pub struct DbConn(diesel::MysqlConnection);

fn rocket() -> rocket::Rocket {
  rocket::ignite()
    .mount("/", routes![
      routes::index,
      routes::run_payments,
    ])
    // .attach(DbConn::fairing())
}

fn main() {
  rocket().launch();
}