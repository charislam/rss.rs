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

impl From<Option<Session>> for UserStore {
    fn from(session: Option<Session>) -> Self {
        match session {
            Some(session) => Self {
                data: Some(UserStoreData::from(session))
            },
            None => Self {
                data: None
            }
        }
    }
}