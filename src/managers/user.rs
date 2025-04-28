use leptos::leptos_dom::logging::console_error;

use crate::auth::client::AuthClient;
use crate::auth::credentials::Credentials;
use crate::stores::user::{User, UserStore};

pub(crate) struct UserManager {
    client: AuthClient
}

impl UserManager {
    pub(crate) fn new() -> Self {
        let client = AuthClient::new();
        Self {
            client
        }
    }

    pub(crate) fn initialize_user_store(&self) -> UserStore {
        let Some(session) = self.client.get_stored_session() else {
            return UserStore::new_empty();
        };
        match Credentials::try_from(&session) {
            Ok(credentials) if credentials.is_valid_loose()  => {
                UserStore::new_user(User::from(session), credentials)
            }
            Ok(_) => {
                UserStore::new_empty()
            }
            Err(error) => {
                console_error(&error);
                UserStore::new_empty()
            }
        }
    }
}