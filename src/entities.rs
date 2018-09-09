use ::schema::project;

#[derive(Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "project"]
pub struct Project {
    pub id: String,
    pub name: String
}