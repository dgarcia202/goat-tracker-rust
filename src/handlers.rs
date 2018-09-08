use rocket_contrib::Json;
use rocket::http::RawStr;
use diesel::prelude::*;

use ::db::DbConn;
use ::entities::{Project, NewProject};
use ::schema::project::dsl::*;

#[get("/")]
pub fn get_projects(conn: DbConn) -> Json<Vec<Project>> {
    let results = project.load::<Project>(&*conn).unwrap();
    Json(results)
} 

#[get("/<project_id>")]
pub fn get_project(project_id: &RawStr, conn: DbConn) -> Json<Project> {
    let result = project.find(project_id.to_string()).first::<Project>(&*conn).unwrap();
    Json(result)
}

#[post("/", format = "application/json", data = "<project_data>")]
pub fn new_project(project_data: Json<NewProject>, conn: DbConn) -> Json<Project> {

    println!("{}", project_data.name);

    let result = project.find("066112f771de41b4aba5648b95476992".to_string()).first::<Project>(&*conn).unwrap();
    Json(result)
}