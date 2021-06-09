#[macro_use]
extern crate rocket;

use rocket::serde::json::json;
use rocket::serde::json::Value;

#[get("/")]
fn hello() -> Value {
    json!({"success": true, "data": "Hello, world!"})
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}
