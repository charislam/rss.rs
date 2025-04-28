use supabase_auth::models::{AuthClient as AuthClientPrimitive, Provider, Session};

use crate::utils::storage::get_local_storage_key;

use super::credentials::Credentials;

const SUPABASE_URL: &'static str = "https://vgiyreydgwouwrzjtgkd.supabase.co";

// !!! IMPORTANT !!!
// This is the _anon_ key, which is safe for client-side use.
const SUPABASE_API_KEY: &'static str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InZnaXlyZXlkZ3dvdXdyemp0Z2tkIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NDU4NzY2ODUsImV4cCI6MjA2MTQ1MjY4NX0.BURrs3NfbKl76fwQ9yn_XrUDZRhh6jLRe2Yao9tnVbI";

// Even though this technically would _not_ be safe for client-side use, we just
// need to provide a dummy string here to prevent errors when intializing the
// Supabase client. The JWT secret is never actually used in code.
const SUPABASE_JWT_SECRET: &'static str = "";

#[derive(Clone, Debug)]
pub(crate) struct AuthClient {
    inner: AuthClientPrimitive,
}

impl AuthClient {
    pub(crate) fn new() -> Self {
        let inner = AuthClientPrimitive::new(SUPABASE_URL, SUPABASE_API_KEY, SUPABASE_JWT_SECRET);
        Self { inner }
    }

    pub(crate) fn get_stored_session(&self) -> Option<Session> {
        get_local_storage_key("supabase.auth.token")
    }

    pub(crate) async fn refresh_session(
        &self,
        credentials: &Credentials,
    ) -> anyhow::Result<Session> {
        self.inner
            .exchange_token_for_session(credentials.refresh_token())
            .await
            .map_err(|e| anyhow::anyhow!("Failed to refresh session: {}", e))
    }

    pub(crate) async fn sign_in_with_github(&self) -> anyhow::Result<()> {
        let oauth_response = self
            .inner
            .login_with_oauth(Provider::Github, None)
            .map_err(|e| anyhow::anyhow!("Failed to sign in with Github: {}", e))?;
        let url = oauth_response.url;
        Ok(())
    }
}
