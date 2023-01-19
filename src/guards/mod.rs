pub mod adminguard;
pub mod authguard;

use rocket_firebase_auth::auth::FirebaseAuth;
pub struct ServerState {
    pub auth: FirebaseAuth,
}
