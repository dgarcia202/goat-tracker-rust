use rocket_contrib::Json;
use rocket::http::RawStr;
use diesel::insert_into;
use diesel::prelude::*;
use uuid::Uuid;

use ::db::DbConn;
use ::entities::Project;
use ::schema::project::dsl::*;

#[get("/")]
pub fn get_all(conn: DbConn) -> Json<Vec<Project>> {
    let results = project.load::<Project>(&*conn).unwrap();
    Json(results)
} 

#[get("/<project_id>")]
pub fn get(project_id: &RawStr, conn: DbConn) -> Json<Project> {
    let result = project.find(project_id.to_string()).first::<Project>(&*conn).unwrap();
    Json(result)
}

#[post("/", format = "application/json", data = "<project_data>")]
pub fn new(project_data: Json<Project>, conn: DbConn) -> Json<Project> {

    let mut new_project = project_data.into_inner();
    new_project.id = Uuid::new_v4().to_string();

    let _ = insert_into(project).values(&new_project).execute(&*conn).unwrap();

    let result = project.find(new_project.id).first::<Project>(&*conn).unwrap();
    Json(result)
}