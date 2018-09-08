use rocket_contrib::Json;
use rocket::http::RawStr;
use diesel::prelude::*;

use ::db::DbConn;
use ::entities::Project;
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