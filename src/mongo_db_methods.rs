use crate::models::*;
use crate::mongo_db_config::connect_to_mongodb;
use mongodb::{
    bson,
    bson::oid::ObjectId,
    bson::{doc, Document},
    error::Error,
    Collection,
};
use rocket::serde::json::json;
use rocket::serde::json::Value;

pub struct MongoDb;

impl MongoDb {
    pub async fn get_collection(name: &str) -> Result<Collection, Error> {
        let client = connect_to_mongodb().await.unwrap();
        let db = client.database("rocket-app");
        let collection = db.collection(name);
        Ok(collection)
    }

    pub async fn insert_one(db: Collection, data: Value) -> Result<Option<MongoRustacean>, Error> {
        let insertable = bson::to_document(&data).unwrap();
        let bson_res = db.insert_one(insertable, None).await.unwrap();
        let res: ObjectId = bson::from_bson(bson_res.inserted_id).unwrap();

        let created_obj = Self::find_one(db.clone(), json!({ "_id": res })).await?;
        Ok(created_obj)
    }

    pub async fn find_one(db: Collection, filter: Value) -> Result<Option<MongoRustacean>, Error> {
        let insertable = bson::to_document(&filter).unwrap();
        let doc_res = db.find_one(insertable, None).await?;
        match doc_res {
            Some(document) => {
                let res = bson::from_bson(bson::Bson::Document(document))?;
                println!("Find: {:#?}", res);
                Ok(res)
            }
            None => Ok(None),
        }
    }

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
