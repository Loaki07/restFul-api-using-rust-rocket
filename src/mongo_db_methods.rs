use crate::models::*;
use crate::mongo_db_config::connect_to_mongodb;
use mongodb::{
    bson,
    bson::oid::ObjectId,
    bson::{doc, Document},
    error::Error,
    Collection,
};
use rocket::serde::json::Json;

pub struct MongoDb;

impl MongoDb {
    pub async fn get_collection(name: &str) -> Result<Collection, Error> {
        let client = connect_to_mongodb().await.unwrap();
        let db = client.database("rocket-app");
        let collection = db.collection(name);
        Ok(collection)
    }

    pub async fn create_one<T>(db: Collection, data: Json<T>)
    where
        T: std::fmt::Debug,
        T: serde::Serialize,
    {
        let insertable = bson::to_document(&data.into_inner()).unwrap();
        let bson_res = db.insert_one(insertable, None).await.unwrap();
        let res: ObjectId = bson::from_bson(bson_res.inserted_id).unwrap();
    }

    // async fn find_one<T>(db: Collection, filter: Json<T>) {
    //     let res = db.find(doc! {filter.into_inner()}, None).await.unwrap();
    //     println!("Find: {:#?}", res);
    // }

    // async fn update_one<T>(db: Collection, query: Json<T>, new_data: Rustacean) {
    //     db.update_one(
    //         doc! {query.into_inner()},
    //         doc! {new_data.into_inner()},
    //         None,
    //     )
    //     .await;
    // }

    // async fn delete_one<T>(db: Collection, query: Json<T>) {
    //     db.delete_one(doc! {query}, None).await;
    // }
}
