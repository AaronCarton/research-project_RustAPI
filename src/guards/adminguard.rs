use crate::guards::ServerState;
use crate::user_service;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome, Request};
use rocket::State;
use rocket_firebase_auth::bearer_token::BearerToken;

pub struct AdminGuard(i32);

#[derive(Debug)]
pub enum AdminGuardError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminGuard {
    type Error = AdminGuardError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // get the server state
        let state = req.guard::<&State<ServerState>>().await.unwrap();

        // get the bearer token from the request and check if it is not empty
        let token = req.guard::<BearerToken>().await;

        // check if token is missing
        if token.is_failure() {
            println!("Authentication failed. Missing token.");
            return Outcome::Failure((Status::Forbidden, AdminGuardError::Missing));
        }

        // check if token is valid
        let jwt = state.auth.verify(&token.unwrap()).await;
        if jwt.is_err() {
            println!("Authentication failed. Invalid token.");
            return Outcome::Failure((Status::Forbidden, AdminGuardError::Invalid));
        }

        // get user from user service
        let user = user_service::get_user_by_uid(jwt.unwrap().uid);
        println!("user: {}", user.uid);

        // check if user is admin
        if user.role != 2 {
            println!("Authentication failed. User is not admin.");
            return Outcome::Failure((Status::Forbidden, AdminGuardError::Invalid));
        }

        // return success
        Outcome::Success(AdminGuard(user.role))
    }
}
