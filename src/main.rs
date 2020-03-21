#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate dotenv_codegen;

pub mod db;
pub mod handlers;
pub mod model;
pub mod schema;

use rocket::config::{Config, Environment, LoggingLevel};

#[get("/")]
fn hello() -> String {
    format!("Hello")
}
fn main() {
    let config = Config::build(Environment::Production)
        .address("127.0.0.1")
        .port(8088)
        .log_level(LoggingLevel::Critical)
        .keep_alive(0)
        .workers(8)
        .unwrap();

    rocket::custom(config)
        .manage(db::connect())
        .mount("/", routes![handlers::users::all])
        .launch();
}
