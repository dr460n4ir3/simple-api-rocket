use rocket::{get, http::Status, serde::json::Json};
use serde::Serialize;

#[macro_use]
extern crate rocket;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[get("/healthchecker")]
pub async fn health_checker_handler() -> Result<Json<GenericResponse>, Status> {
    const MESSAGE: &str = "Build a Simple API with Rocket.rs";
    
    let response_json = GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    Ok(Json(response_json))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![health_checker_handler])
}