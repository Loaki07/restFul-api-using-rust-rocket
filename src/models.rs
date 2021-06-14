use super::schema::rustaceans;
use mongodb::bson::oid::ObjectId;

#[derive(Queryable, serde::Serialize, serde::Deserialize, AsChangeset, Clone, Debug)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_deserializing)]
    pub created_at: String,
}

#[derive(Insertable, serde::Deserialize, Debug, serde::Serialize, Clone)]
#[table_name = "rustaceans"]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}

#[derive(serde::Deserialize, Debug, serde::Serialize, Clone)]
pub struct FindById {
    pub _id: ObjectId,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct MongoRustacean {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: ObjectId,
    pub name: String,
    pub email: String,
    #[serde(skip_deserializing)]
    pub created_at: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct InsertableMongoRustacean {
    pub name: String,
    pub email: String,
}
