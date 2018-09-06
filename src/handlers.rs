use rocket_contrib::Json;

use ::db;
use ::entities::Project;
use ::schema::project::dsl::*;

// #[get("/")]
// pub fn index() -> Json<Project> {
//     Json(Project { id: 1, name: "dddd".to_string(), description: "fsgfsg".to_string() })
// } 


#[get("/")]
pub fn index() -> Json<Project> {

    let conn = db::establish_connection();
    // let result = project
    //             load::<Project>(&conn).
    //             expect("Error loading projects");

    let result = project.filter(name.eq("aaa")).load::<Project>(&conn);
    Json(result)
} 