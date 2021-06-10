#[macro_use]
extern crate rocket;
mod auth;

use auth::BasicAuth;
use rocket::response::status;
use rocket::serde::json::json;
use rocket::serde::json::Value;
use rocket_sync_db_pools::{database, diesel};

#[database("sqlite_db")]
struct DbConn(diesel::SqliteConnection);

#[get("/")]
fn hello() -> Value {
    json!({"success": true, "data": "Hello, world!"})
}

#[get("/rustaceans")]
fn get_rustacean(auth: BasicAuth, _conn: DbConn) -> Value {
    println!("{:#?}", auth);
    json!([
        {
            "id": 1,
            "name": "John Doe"
        },
        {
            "id": 2,
            "name": "Adam"
        },
        {
            "id": 3,
            "name": "Eve"
        },
    ])
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
