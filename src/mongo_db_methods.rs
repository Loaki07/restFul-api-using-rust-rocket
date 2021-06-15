use crate::models::*;
use crate::mongo_db_config::connect_to_mongodb;
use mongodb::{
    bson,
    bson::oid::ObjectId,
    bson::{doc, Document},
    error::Error,
    options::UpdateModifications,
    results::DeleteResult,
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

    pub async fn update_one(
        db: Collection,
        filter: Value,
        new_data: InsertableMongoRustacean,
    ) -> Result<Option<MongoRustacean>, Error> {
        println!("new_data: {:#?}", &new_data);
        let insertable_filter = bson::to_document(&filter).unwrap();
        println!("insertable_filter: {:#?}", insertable_filter.clone());

        // let new_data_to_json = json!({ "$set": {
        //     "name": new_data.name,
        //     "email": new_data.email
        // } });
        // println!("new_data: {:#?}", &new_data_to_json);
        // let insertable_new_data = bson::to_document(&new_data_to_json).unwrap();
        // println!("insertable_new_data: {:#?}", insertable_new_data.clone());

        let doc_res = db
            .update_one(
                insertable_filter,
                doc! { "$set": {
                    "name": new_data.name,
                    "email": new_data.email
                } },
                None,
            )
            .await?;
        println!("Updated {} document", doc_res.modified_count);
        // match doc_res {
        //     Ok(document) => {
        //         let res = bson::from_bson(bson::Bson::Document(document))?;
        //         println!("Update: {:#?}", res);
        //         Ok(res)
        //     }
        //     Err(_) => Ok(None),
        // }
        Ok(None)
    }

    pub async fn delete_one(db: Collection, id: String) -> Result<DeleteResult, Error> {
        let filter_json = json!({ "_id": {
            "$oid": id
        } });
        let insertable_filter = bson::to_document(&filter_json).unwrap();
        println!("insertable_filter: {:#?}", insertable_filter.clone());
        db.delete_one(insertable_filter, None).await
    }
}
