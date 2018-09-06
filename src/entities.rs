#[allow(proc_macro_derive_resolution_fallback)]

#[derive(Serialize, Queryable)]
pub struct Project {
    pub id: String,
    pub name: String
}