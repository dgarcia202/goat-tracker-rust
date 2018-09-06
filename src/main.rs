#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate dotenv;

pub mod db;
pub mod entities;
pub mod handlers;
pub mod schema;

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![handlers::index])
}

fn main() {
    rocket().launch();
}