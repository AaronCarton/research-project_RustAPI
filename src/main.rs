#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;
mod db;
mod models;
mod schema;
mod user_service;
use diesel_migrations::embed_migrations;
use models::{NewUser, User};
use rocket::fs::{relative, FileServer};
use rocket::serde::json::Json;

embed_migrations!();

#[get("/test")]
fn index() -> &'static str {
    "hello world"
}

#[post("/users", data = "<payload>")]
fn create_user(payload: Json<NewUser>) -> Json<User> {
    let user = payload.into_inner();
    Json(user_service::create_user(user))
}

#[get("/users")]
fn get_users() -> Json<Vec<User>> {
    Json(user_service::get_users())
}

#[get("/users/<id>")]
fn get_user(id: i32) -> Json<User> {
    Json(user_service::get_user(id))
}

#[delete("/users/<id>")]
fn delete_user(id: i32) {
    user_service::delete_user(id);
}

#[put("/users", data = "<user>")]
fn update_user(user: Json<User>) -> Json<User> {
    Json(user_service::update_user(user.into_inner()))
}

#[launch]
fn rocket() -> _ {
    match embedded_migrations::run(&db::create_connection()) {
        Ok(_) => rocket::build().mount(
            "/api",
            routes![
                index,
                create_user,
                get_user,
                get_users,
                delete_user,
                update_user
            ],
        ),
        Err(_) => panic!("migration failed"),
    }
}
