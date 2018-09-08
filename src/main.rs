#![feature(plugin)]
#![plugin(rocket_codegen)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate dotenv;
extern crate uuid;

pub mod db;
pub mod entities;
pub mod handlers;
pub mod schema;

use handlers::*;

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .manage(db::init_pool())
        .mount("/projects", routes![project::get_all, project::get, project::new])
}

fn main() {
    rocket().launch();
}