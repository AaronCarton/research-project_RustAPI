#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;
mod authguard;
mod db;
mod models;
mod roleguard;
mod schema;
mod user_service;
use authguard::AuthGuard;
use db::establish_connection;
use diesel_migrations::embed_migrations;
use models::{NewUser, User};
use rocket::routes;
use rocket::serde::json::Json;
use rocket::{http::Status, response::status, State};
use rocket_firebase_auth::{auth::FirebaseAuth, bearer_token::BearerToken};
use roleguard::RoleGuard;

pub struct ServerState {
    pub auth: FirebaseAuth,
}


#[get("/")]
async fn hello_world(_auth: AuthGuard, _role: RoleGuard) -> status::Accepted<String> {
    // state: &State<ServerState>, token: BearerToken

    // match state
    //     .auth
    //     .verify(&token).await// verify token

    // {
    //     Ok(token) => {
    //         let uid = token.uid;
    //         println!("Authentication succeeded with uid={uid}");
    //         Status::Ok
    //     }
    //     Err(_) => {
    //         println!("Authentication failed.");
    //         Status::Forbidden
    //     }
    // }
    status::Accepted(Some("Hello, world!".to_string()))
}

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

#[put("/users/<id>/increase_score")]
fn update_user(id: i32) -> Json<User> {
    Json(user_service::increase_score(id))
}

#[launch]
fn rocket() -> _ {
    let firebase_auth = FirebaseAuth::try_from_json_file("firebase-credentials.json")
        .expect("Failed to read Firebase credentials");

    rocket::build()
        .mount(
            "/api",
            routes![
                hello_world,
                index,
                create_user,
                get_user,
                get_users,
                delete_user,
                update_user
            ],
        )
        .manage(ServerState {
            auth: firebase_auth,
        })
}
