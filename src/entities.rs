use ::schema::project;

#[derive(Deserialize, Serialize, Queryable, Insertable, Identifiable, AsChangeset)]
#[table_name = "project"]
pub struct Project {
    pub id: String,
    pub name: String
}