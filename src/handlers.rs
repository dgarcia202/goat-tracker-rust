use rocket_contrib::Json;
use rocket::http::RawStr;
use diesel::insert_into;
use diesel::prelude::*;
use uuid::Uuid;

use ::db::DbConn;
use ::entities::{Project, NewProject};
use ::schema::project::dsl::*;

#[get("/")]
pub fn get_project_list(conn: DbConn) -> Json<Vec<Project>> {
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

    let new_id = Uuid::new_v4().to_string();

    let _ = insert_into(project)
                .values((id.eq(new_id.clone()), name.eq(project_data.name.clone())))
                .execute(&*conn).unwrap();

    let result = project.find(new_id).first::<Project>(&*conn).unwrap();
    Json(result)
}