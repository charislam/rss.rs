use leptos::{leptos_dom::logging::console_error, prelude::*};
use reactive_stores::Store;

use crate::auth::client::AuthClient;
use crate::auth::credentials::Credentials;
use crate::stores::user::{User, UserStore, UserStoreStoreFields};

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

    async fn refresh_session(&self, user_store: &Store<UserStore>) {
        if user_store.credentials().read_untracked().is_some() {
            if let Some(credentials) = user_store.credentials().read_untracked().as_ref() {
                match self.client.refresh_session(credentials).await {
                    Ok(session) => {
                        let new_credentials = Credentials::try_from(&session);
                        if let Ok(new_credentials) = new_credentials {
                            user_store.credentials().write().replace(new_credentials);
                            user_store.user().write().replace(User::from(session));
                        } else {
                            console_error("Failed to refresh session: invalid credentials");
                        }
                    }
                    Err(error) => {
                        console_error(&error.to_string());
                    }
                }
            }
        }
    }
}