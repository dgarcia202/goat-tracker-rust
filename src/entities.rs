use ::schema::*;

#[derive(Deserialize, Serialize, Queryable, Insertable, Identifiable, AsChangeset)]
#[table_name = "project"]
pub struct Project {
    pub id: String,
    pub name: String
}

#[derive(Deserialize, Serialize, Queryable, Insertable, Identifiable, AsChangeset)]
#[table_name = "release"]
pub struct Release {
    pub id: String,
    pub project_id: String,
    pub version: String
}