#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
mod auth;
mod models;
mod schema;

use diesel::prelude::*;
use models::*;
use schema::*;
use auth::BasicAuth;
use rocket::response::status;
use rocket::serde::json::json;
use rocket::serde::json::Value;
use rocket_sync_db_pools::database;

#[database("sqlite_db")]
struct DbConn(diesel::SqliteConnection);

#[get("/")]
fn hello() -> Value {
    json!({"success": true, "data": "Hello, world!"})
}

#[get("/rustaceans")]
async fn get_rustacean(auth: BasicAuth, conn: DbConn) -> Value {
    println!("{:#?}", auth);
    conn.run(|c| {
        let all = rustaceans::table
            .limit(100)
            .load::<Rustacean>(c)
            .expect("Error loading rustaceans from DB!");
        json!(all)
    })
    .await
}

#[get("/rustaceans/<id>")]
fn view_rustacean(id: i32, _auth: BasicAuth) -> Value {
    json!({
        "id": id,
        "name": "John Doe",
        "email": "john@doe.com"
    })
}

#[post("/rustaceans", format = "json")]
fn create_rustacean(_auth: BasicAuth) -> Value {
    json!({
        "id": 1,
        "name": "John Doe",
        "email": "john@doe.com"
    })
}

#[put("/rustaceans/<id>", format = "json")]
fn update_rustacean(id: i32, _auth: BasicAuth) -> Value {
    json!({
        "id": id,
        "name": "John Doe",
        "email": "john@doe.com"
    })
}

#[delete("/rustaceans/<_id>")]
fn delete_rustacean(_id: i32, _auth: BasicAuth) -> status::NoContent {
    status::NoContent
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
