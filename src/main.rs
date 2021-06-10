#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
mod auth;
mod models;
mod schema;

use auth::BasicAuth;
use diesel::prelude::*;
use models::*;
use rocket::response::status;
use rocket::serde::json::json;
use rocket::serde::json::Json;
use rocket::serde::json::Value;
use rocket_sync_db_pools::database;
use schema::*;

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
async fn view_rustacean(id: i32, _auth: BasicAuth, conn: DbConn) -> Value {
    conn.run(move |c| {
        let rustacean = rustaceans::table
            .find(id)
            .get_result::<Rustacean>(c)
            .expect("Error loading rustacean from DB");
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
        let result = diesel::insert_into(rustaceans::table)
            .values(new_rustacean.into_inner())
            .execute(c)
            .expect("Error adding rustaceans to DB");
        json!(result)
    })
    .await
}

#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
async fn update_rustacean(
    id: i32,
    _auth: BasicAuth,
    conn: DbConn,
    rustacean: Json<Rustacean>,
) -> Value {
    conn.run(move |c| {
        let result = diesel::update(rustaceans::table.find(id))
            .set((
                rustaceans::name.eq(rustacean.name.to_owned()),
                rustaceans::email.eq(rustacean.email.to_owned()),
            ))
            .execute(c)
            .expect("Error updating rustaceans to DB");
        json!(result)
    })
    .await
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
