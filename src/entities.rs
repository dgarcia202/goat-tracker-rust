#[derive(Serialize, Queryable)]
pub struct Project {
    pub id: String,
    pub name: String
}

#[derive(Deserialize)]
pub struct NewProject {
    pub name: String
}