use crate::models::user::{NewUser, User, UserModel};
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
pub fn get_user(id: i32) -> Json<UserModel> {
    Json(user_service::get_user(id))
}

#[delete("/reset/<uid>")]
pub fn reset_user(uid: String) -> Json<UserModel> {
    Json(user_service::reset_user(uid))
}
