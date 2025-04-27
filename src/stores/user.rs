use reactive_stores::Store;
use supabase_auth::models::Session;

use crate::auth::client::AuthClient;
use crate::auth::credentials::Credentials;

#[derive(Clone, Debug, Store)]
pub(crate) struct UserStore {
    client: AuthClient,
    pub(crate) data: Option<UserStoreData>,
}

#[derive(Clone, Debug, Store)]
pub(crate) struct UserStoreData {
    pub(crate) user: User,
    credentials: Credentials,
}

#[derive(Clone, Debug, Store)]
pub(crate) struct User {
    pub(crate) username: Option<String>,
    pub(crate) email: String,
}

impl UserStore {
    pub(crate) fn new() -> Self {
        let client = AuthClient::new();
        let data = AuthClient::get_stored_session().map(UserStoreData::from);
        Self {
            client,
            data
        }
    }
}

impl From<Session> for UserStoreData {
    fn from(session: Session) -> Self {
        Self {
            user: User {
                username: session.user.user_metadata.name,
                email: session.user.email,
            },
            credentials: Credentials::new(session.access_token, session.refresh_token)
        }
    }
}