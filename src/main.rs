#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod auth;
mod models;
mod repositories;
mod schema;

use auth::BasicAuth;
use models::*;
use repositories::*;
use rocket::response::status;
use rocket::serde::json::json;
use rocket::serde::json::Json;
use rocket::serde::json::Value;
use rocket_sync_db_pools::database;

#[database("sqlite_db")]
struct DbConn(diesel::SqliteConnection);

#[get("/")]
fn hello() -> Value {
    json!({"success": true, "data": "Hello, world!"})
}

#[get("/rustaceans")]
async fn get_rustacean(_auth: BasicAuth, conn: DbConn) -> Value {
    conn.run(|c| {
        let all = RustaceanRepository::load_all(c).expect("Error loading rustaceans from DB!");
        json!(all)
    })
    .await
}

#[get("/rustaceans/<id>")]
async fn view_rustacean(id: i32, _auth: BasicAuth, conn: DbConn) -> Value {
    conn.run(move |c| {
        let rustacean = RustaceanRepository::find(c, id).expect("Error loading rustacean from DB");
        json!(rustacean)
    })
    .await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustacean(
    _auth: BasicAuth,
    conn: DbConn,
    new_rustacean: Json<NewRustacean>,
) -> Value {
    conn.run(|c| {
        let result = RustaceanRepository::create(c, new_rustacean.into_inner())
            .expect("Error adding rustaceans to DB");
        json!(result)
    })
    .await
}

#[put("/rustaceans/<_id>", format = "json", data = "<rustacean>")]
async fn update_rustacean(
    _id: i32,
    _auth: BasicAuth,
    conn: DbConn,
    rustacean: Json<Rustacean>,
) -> Value {
    conn.run(move |c| {
        let result = RustaceanRepository::save(c, rustacean.into_inner())
            .expect("Error updating rustaceans to DB");
        json!(result)
    })
    .await
}

#[delete("/rustaceans/<id>")]
async fn delete_rustacean(id: i32, _auth: BasicAuth, conn: DbConn) -> status::NoContent {
    conn.run(move |c| {
        RustaceanRepository::delete(c, id).expect("Error deleting rustacean from DB");
        status::NoContent
    })
    .await
}

#[catch(404)]
fn not_found() -> Value {
    json!({ "success": false, "data": "Not found!"})
}

/**
 * Authorization Header: Authorization: Basic QWxhZGRpbjpPcGVuU2VzYW1l
 */
#[launch]
fn rocket() -> _ {
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
            ],
        )
        .register("/", catchers![not_found])
        .attach(DbConn::fairing())
}
