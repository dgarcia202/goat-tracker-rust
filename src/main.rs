#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate serde_derive;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;

pub mod entities;
pub mod handlers;

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![handlers::index])
}

fn main() {
    rocket().launch();
}