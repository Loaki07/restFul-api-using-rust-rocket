#[macro_use]
extern crate rocket;

use rocket::response::status;
use rocket::serde::json::json;
use rocket::serde::json::Value;

#[get("/")]
fn hello() -> Value {
    json!({"success": true, "data": "Hello, world!"})
}

#[get("/rustaceans")]
fn get_rustacean() -> Value {
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
fn view_rustacean(id: i32) -> Value {
    json!({
        "id": id,
        "name": "John Doe",
        "email": "john@doe.com"
    })
}

#[post("/rustaceans", format = "json")]
fn create_rustacean() -> Value {
    json!({
        "id": 1,
        "name": "John Doe",
        "email": "john@doe.com"
    })
}

#[put("/rustaceans/<id>", format = "json")]
fn update_rustacean(id: i32) -> Value {
    json!({
        "id": id,
        "name": "John Doe",
        "email": "john@doe.com"
    })
}

#[delete("/rustaceans/<_id>")]
fn delete_rustacean(_id: i32) -> status::NoContent {
    status::NoContent
}

#[catch(404)]
fn not_found() -> Value {
    json!({ "success": false, "data": "Not found!"})
}

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
}
