use crate::guards::adminguard::AdminGuard;
use crate::guards::authguard::AuthGuard;
use crate::models::user::{NewUser, User, UserModel};
use crate::services::user_service;
use rocket::serde::json::Json;

#[post("/", data = "<payload>")]
pub fn create_user(auth: AuthGuard, payload: Json<NewUser>) -> Json<User> {
    let mut user = payload.into_inner();
    user.uid = Some(auth.uid);
    Json(user_service::create_user(user))
}

#[get("/")]
pub fn get_users(_admin: AdminGuard) -> Json<Vec<User>> {
    Json(user_service::get_users())
}

#[get("/<id>")]
pub fn get_user(_admin: AdminGuard, id: i32) -> Json<UserModel> {
    Json(user_service::get_user(id))
}

#[get("/self")]
pub fn get_self(auth: AuthGuard) -> Json<UserModel> {
    Json(user_service::get_user_by_uid(auth.uid))
}

#[delete("/self/reset")]
pub fn reset_user(auth: AuthGuard) -> Json<UserModel> {
    Json(user_service::reset_user(auth.uid))
}
