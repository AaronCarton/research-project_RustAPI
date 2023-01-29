#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
mod guards;
use crate::guards::ServerState;
use guards::{adminguard::AdminGuard, authguard::AuthGuard};
mod db;
mod models;
mod routes;

use rocket::serde::json::json;
use routes::{quiz_routes::*, user_routes::*};
mod schema;
mod services;
use rocket::response::status;
use rocket::routes;
use rocket_firebase_auth::auth::FirebaseAuth;
use services::user_service;

#[get("/")]
async fn hello_world(_auth: AuthGuard, _admin: AdminGuard) -> status::Accepted<String> {
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

// forbidden cather with error json response
#[catch(403)]
fn forbidden() -> serde_json::Value {
    json!({
        "error": "Forbidden",
        "message":     "You are not allowed to access this resource. Either you did not provide a valid token or you are not authorized to access this resource."

    })
}

#[launch]
fn rocket() -> _ {
    let firebase_auth = FirebaseAuth::try_from_json_file("firebase-credentials.json")
        .expect("Failed to read Firebase credentials");

    // set CORS headers

    rocket::build()
        .attach(
            rocket_cors::CorsOptions {
                allowed_origins: rocket_cors::AllowedOrigins::all(),
                allowed_methods: vec![
                    rocket::http::Method::Get,
                    rocket::http::Method::Post,
                    rocket::http::Method::Put,
                    rocket::http::Method::Delete,
                ]
                .into_iter()
                .map(From::from)
                .collect(),
                allowed_headers: rocket_cors::AllowedHeaders::all(),
                allow_credentials: true,
                ..Default::default()
            }
            .to_cors()
            .expect("Failed to create CORS"),
        )
        .mount("/api", routes![hello_world, index,])
        .mount(
            "/api/users",
            routes![create_user, get_users, get_user, get_self, reset_user],
        )
        .mount(
            "/api/quiz",
            routes![
                create_quiz,
                get_quizs,
                get_quiz,
                answer_quiz_question,
                get_quiz_score
            ],
        )
        .register("/", catchers![forbidden])
        .manage(ServerState {
            auth: firebase_auth,
        })
}
