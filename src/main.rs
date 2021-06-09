#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::status;
use rocket::serde::json::json;
use rocket::serde::json::Value;

#[get("/")]
fn hello() -> Value {
    json!({"success": true, "data": "Hello, world!"})
}

#[get("/rustaceans")]
fn get_rustacean(auth: BasicAuth) -> Value {
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

#[derive(Debug)]
pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

impl BasicAuth {
    fn from_authorization_header(header: &str) -> Option<BasicAuth> {
        let split = header.split_whitespace().collect::<Vec<_>>();

        if split.len() != 2 {
            return None;
        }

        if split[0] != "Basic" {
            return None;
        }

        Self::from_base64_encoded(split[1])
    }

    fn from_base64_encoded(base64_string: &str) -> Option<BasicAuth> {
        let decoded = base64::decode(base64_string).ok()?;
        let decoded_str = String::from_utf8(decoded).ok()?;
        let split = decoded_str.split(":").collect::<Vec<_>>();

        // If exactly username & password pair are present
        if split.len() != 2 {
            return None;
        }

        let (username, password) = (split[0].to_string(), split[1].to_string());

        Some(BasicAuth { username, password })
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");
        if let Some(auth_header) = auth_header {
            if let Some(auth) = Self::from_authorization_header(auth_header) {
                return Outcome::Success(auth);
            }
        }

        Outcome::Failure((Status::Unauthorized, ()))
    }
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
}
