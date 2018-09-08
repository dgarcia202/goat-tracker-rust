use rocket_contrib::Json;
use rocket::http::RawStr;
use diesel::prelude::*;

use ::db;
use ::entities::Project;
use ::schema::project::dsl::*;

#[get("/")]
pub fn get_projects() -> Json<Vec<Project>> {
    let conn = db::establish_connection();
    let results = project.load::<Project>(&conn).unwrap();
    Json(results)
} 

#[get("/<project_id>")]
pub fn get_project(project_id: &RawStr) -> Json<Project> {
    let conn = db::establish_connection();
    let result = project.find(project_id.to_string()).first::<Project>(&conn).unwrap();
    Json(result)
} 