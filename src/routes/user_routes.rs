use crate::models::user::{NewUser, User};
use crate::services::user_service;
use rocket::serde::json::Json;

#[post("/", data = "<payload>")]
pub fn create_user(payload: Json<NewUser>) -> Json<User> {
    let user = payload.into_inner();
    Json(user_service::create_user(user))
}

#[get("/")]
pub fn get_users() -> Json<Vec<User>> {
    Json(user_service::get_users())
}

#[get("/<id>")]
pub fn get_user(id: i32) -> Json<User> {
    Json(user_service::get_user(id))
}

#[delete("/<id>")]
pub fn delete_user(id: i32) {
    user_service::delete_user(id);
}

#[put("/<id>/increase_score")]
pub fn increase_score(id: i32) -> Json<User> {
    Json(user_service::increase_score(id))
}
