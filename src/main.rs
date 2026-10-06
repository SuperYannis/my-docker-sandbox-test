#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct HealthResponse {
    status: &'static str,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct GreetRequest {
    name: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct GreetResponse {
    message: String,
}

#[get("/health")]
fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

#[post("/greet", format = "json", data = "<req>")]
fn greet(req: Json<GreetRequest>) -> Json<GreetResponse> {
    Json(GreetResponse {
        message: format!("Hello, {}!", req.name),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![health, greet])
}

#[cfg(test)]
mod tests {
    use super::rocket;
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;

    fn client() -> Client {
        Client::tracked(rocket()).expect("valid rocket instance")
    }

    #[test]
    fn health_returns_ok() {
        let client = client();
        let response = client.get("/health").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), r#"{"status":"ok"}"#);
    }

    #[test]
    fn greet_returns_greeting() {
        let client = client();
        let response = client
            .post("/greet")
            .header(ContentType::JSON)
            .body(r#"{"name":"Alice"}"#)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().unwrap(),
            r#"{"message":"Hello, Alice!"}"#
        );
    }

    #[test]
    fn greet_rejects_missing_name() {
        let client = client();
        let response = client
            .post("/greet")
            .header(ContentType::JSON)
            .body(r#"{}"#)
            .dispatch();
        assert_eq!(response.status(), Status::UnprocessableEntity);
    }
}
