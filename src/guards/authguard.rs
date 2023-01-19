use crate::guards::ServerState;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome, Request};
use rocket::State;
use rocket_firebase_auth::auth::DecodedToken;
use rocket_firebase_auth::bearer_token::BearerToken;

pub struct AuthGuard(DecodedToken);

#[derive(Debug)]
pub enum AuthGuardError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthGuard {
    type Error = AuthGuardError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // get the server state
        let state = req.guard::<&State<ServerState>>().await.unwrap();

        // get the bearer token from the request and check if it is not empty
        let token = req.guard::<BearerToken>().await;

        // check if token is missing
        if token.is_failure() {
            println!("Authentication failed. Missing token.");
            return Outcome::Failure((Status::Forbidden, AuthGuardError::Missing));
        }

        // verify the token
        match state.auth.verify(&token.unwrap()).await {
            Ok(token) => {
                let uid = &token.uid;
                println!("Authentication succeeded with uid={}", uid);
                Outcome::Success(AuthGuard(token))
            }
            Err(_) => {
                println!("Authentication failed.");
                Outcome::Failure((Status::Forbidden, AuthGuardError::Invalid))
            }
        }
    }
}
