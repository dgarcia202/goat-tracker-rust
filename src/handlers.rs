use rocket_contrib::Json;
use ::entities::Project;

#[get("/")]
pub fn index() -> Json<Project> {
    Json(Project { id: 1, name: "dddd".to_string(), description: "fsgfsg".to_string() })
} 