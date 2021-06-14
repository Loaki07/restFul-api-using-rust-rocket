use super::schema::rustaceans;

#[derive(Queryable, serde::Serialize, serde::Deserialize, AsChangeset)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_deserializing)]
    pub created_at: String,
}

#[derive(Insertable, serde::Deserialize, Debug, serde::Serialize)]
#[table_name = "rustaceans"]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}
