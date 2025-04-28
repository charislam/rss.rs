use reactive_stores::Store;
use supabase_auth::models::Session;

use crate::auth::credentials::Credentials;

#[derive(Clone, Debug, Store)]
pub(crate) struct UserStore {
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
    pub(crate) fn new_empty() -> Self {
        Self { data: None }
    }

    pub(crate) fn new_user(user: User, credentials: Credentials) -> Self {
        Self {
            data: Some(UserStoreData {
                user,
                credentials,
            }),
        }
    }
}

impl From<Session> for User {
    fn from(session: Session) -> Self {
        Self {
            username: session.user.user_metadata.name,
            email: session.user.email,
        }
    }
}
