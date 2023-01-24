pub mod auth;

use crate::context::store::Store;

use crate::config::Config;

#[derive(Clone)]
pub struct Services {
    pub auth: auth::AuthService,
}

impl Services {
    pub fn new(config: &Config, store: Store) -> Self {
        let auth = auth::AuthService::new(config.jwt_secret.to_string());

        Self { auth }
    }
}
