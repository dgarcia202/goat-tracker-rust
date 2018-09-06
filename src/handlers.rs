use rocket_contrib::Json;
use diesel::prelude::*;

use ::db;
use ::entities::Project;
use ::schema::project::dsl::*;

// #[get("/")]
// pub fn index() -> Json<Project> {
//     Json(Project { id: 1, name: "dddd".to_string(), description: "fsgfsg".to_string() })
// } 


#[get("/")]
pub fn index() -> Json<Vec<Project>> {
    let conn = db::establish_connection();
    let results = project.filter(name.eq("aaa".to_string())).load::<Project>(&conn).unwrap();
    Json(results)
} 