use rocket_contrib::Json;
use diesel::prelude::*;

use ::db;
use ::entities::Project;
use ::schema::project::dsl::*;

#[get("/")]
pub fn index() -> Json<Vec<Project>> {
    let conn = db::establish_connection();
    let results = project.load::<Project>(&conn).unwrap();
    Json(results)
} 