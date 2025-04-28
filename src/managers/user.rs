use crate::auth::client::AuthClient;
use crate::stores::user::UserStore;

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
        UserStore::from(self.client.get_stored_session())
    }
}