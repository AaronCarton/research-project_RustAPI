use crate::guards::ServerState;
use rocket::http::Status;
use rocket::request::local_cache;
use rocket::request::{self, FromRequest, Outcome, Request};
use rocket::response::content::Json;
use rocket::State;
use rocket_firebase_auth::bearer_token::BearerToken;

pub struct AuthGuard {
    pub uid: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthGuard {
    type Error = Json<&'static str>;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // get the server state
        let state = req.guard::<&State<ServerState>>().await.unwrap();

        // get the bearer token from the request
        let token = req.guard::<BearerToken>().await;

        // check if token is missing
        if token.is_failure() {
            println!("Authentication failed. Missing token.");
            return Outcome::Failure((
                Status::Forbidden,
                Json("Authentication failed. Missing token."),
            ));
        }

        // verify the token
        match state.auth.verify(&token.unwrap()).await {
            Ok(token) => {
                let uid = &token.uid;
                println!("Authentication succeeded with uid={}", uid);
                Outcome::Success(AuthGuard { uid: uid.clone() })
            }
            Err(_) => {
                println!("Authentication failed.");
                // store the error message in the local cache under the name "error_message" so it can be used in the default catcher
                local_cache!(req, String::from("Authentication failed. Invalid token."));
                Outcome::Failure((
                    Status::Forbidden,
                    Json("Authentication failed. Invalid token."),
                ))
            }
        }
    }
}
