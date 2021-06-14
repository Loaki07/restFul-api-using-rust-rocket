#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod auth;
mod models;
mod mongo_db_config;
mod mongo_db_methods;
mod repositories;
mod schema;

use auth::BasicAuth;
use models::*;
use mongo_db_config::connect_to_mongodb;
use mongo_db_methods::MongoDb;
use mongodb::bson::{doc, Document};
use repositories::*;
// use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::json;
use rocket::serde::json::Json;
use rocket::serde::json::Value;
// use rocket::Phase;
use rocket_sync_db_pools::database;

embed_migrations!();

#[database("sqlite_db")]
struct DbConn(diesel::SqliteConnection);

#[get("/")]
fn hello() -> Value {
    json!({"success": true, "data": "Hello, world!"})
}

#[get("/rustaceans")]
async fn get_rustacean(_auth: BasicAuth, conn: DbConn) -> Result<Value, status::Custom<Value>> {
    conn.run(|c| {
        RustaceanRepository::load_all(c)
            .map(|rustaceans| json!(rustaceans))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[get("/rustaceans/<id>")]
async fn view_rustacean(
    id: i32,
    _auth: BasicAuth,
    conn: DbConn,
) -> Result<Value, status::Custom<Value>> {
    conn.run(move |c| {
        RustaceanRepository::find(c, id)
            .map(|rustacean| json!(rustacean))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustacean(
    _auth: BasicAuth,
    conn: DbConn,
    new_rustacean: Json<NewRustacean>,
) -> Result<Value, status::Custom<Value>> {
    conn.run(|c| {
        RustaceanRepository::create(c, new_rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[put("/rustaceans/<_id>", format = "json", data = "<rustacean>")]
async fn update_rustacean(
    _id: i32,
    _auth: BasicAuth,
    conn: DbConn,
    rustacean: Json<Rustacean>,
) -> Result<Value, status::Custom<Value>> {
    conn.run(move |c| {
        RustaceanRepository::save(c, rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[delete("/rustaceans/<id>")]
async fn delete_rustacean(
    id: i32,
    _auth: BasicAuth,
    conn: DbConn,
) -> Result<status::NoContent, status::Custom<Value>> {
    conn.run(move |c| {
        RustaceanRepository::delete(c, id)
            .map(|_rustacean| status::NoContent)
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[catch(404)]
fn not_found() -> Value {
    json!({ "success": false, "data": "Not found!"})
}

// async fn run_db_migrations<P>(
//     rocket: rocket::Rocket<P>,
// ) -> Result<rocket::Rocket<P>, rocket::Rocket<P>>
// where
//     P: Phase,
// {
//     DbConn::get_one(&rocket)
//         .await
//         .expect("failed to retriever database connection")
//         .run(|c| match embedded_migrations::run(c) {
//             Ok(()) => Ok(rocket),
//             Err(e) => {
//                 println!("Failed to run database migrations: {:?}", e);
//                 Err(rocket)
//             }
//         })
//         .await
// }

#[post("/api/create", format = "json", data = "<new_rustacean>")]
async fn create(new_rustacean: Json<NewRustacean>) -> Value {
    let collection = MongoDb::get_collection("rocket-app").await.unwrap();
    MongoDb::insert_one(collection, new_rustacean).await;
    json! ({ "success": true, "data": "Insert_one" })
}

/**
 * Authorization Header: Authorization: Basic QWxhZGRpbjpPcGVuU2VzYW1l
 */
#[launch]
async fn rocket() -> _ {
    let _mongo_client = connect_to_mongodb().await;
    rocket::build()
        .mount(
            "/",
            routes![
                hello,
                get_rustacean,
                create_rustacean,
                view_rustacean,
                update_rustacean,
                delete_rustacean,
                create,
            ],
        )
        .register("/", catchers![not_found])
        .attach(DbConn::fairing())
    // .attach(AdHoc::on_ignite("Database Migrations", run_db_migrations))
}
