use supabase_auth::models::{AuthClient as AuthClientPrimitive, Session};

use crate::utils::storage::get_local_storage_key;

const SUPABASE_URL: &'static str = "";

// !!! IMPORTANT !!!
// This is the _anon_ key, which is safe for client-side use.
const SUPABASE_API_KEY: &'static str = "";

// Even though this technically would _not_ be safe for client-side use, we just
// need to provide a dummy string here to prevent errors when intializing the
// Supabase client. The JWT secret is never actually used in code.
const SUPABASE_JWT_SECRET: &'static str = "";

#[derive(Clone, Debug)]
pub(crate) struct AuthClient {
    inner: AuthClientPrimitive
}

impl AuthClient {
    pub(crate) fn new() -> Self {
        let inner = AuthClientPrimitive::new(
            SUPABASE_URL,
            SUPABASE_API_KEY,
            SUPABASE_JWT_SECRET
        );
        Self {
            inner
        }
    }

    pub(crate) fn get_stored_session(&self) -> Option<Session> {
        get_local_storage_key("supabase.auth.token")
    }
}