use chrono::{DateTime, Duration, Utc};
use supabase_auth::models::Session;

#[derive(Clone, Debug)]
pub(crate) struct Credentials {
    access_token: String,
    refresh_token: String,
    expires_at: DateTime<Utc>,
}

pub(crate) enum CredentialsStatus {
    Valid,
    Expired,
    ExpiringSoon,
}

impl Credentials {
    pub(crate) fn new(
        access_token: String,
        refresh_token: String,
        unix_time_seconds: impl TryInto<i64>,
    ) -> Option<Self> {
        let unix_time_seconds = unix_time_seconds.try_into().ok()?;
        let time = DateTime::<Utc>::from_timestamp(unix_time_seconds, 0)?;
        Some(Self {
            access_token,
            refresh_token,
            expires_at: time,
        })
    }

    pub(crate) fn status(&self) -> CredentialsStatus {
        let now = Utc::now();
        if self.expires_at <= now {
            CredentialsStatus::Expired
        } else if self.expires_at - now < Duration::minutes(5) {
            CredentialsStatus::ExpiringSoon
        } else {
            CredentialsStatus::Valid
        }
    }

    pub(crate) fn is_valid_loose(&self) -> bool {
        matches!(self.status(), CredentialsStatus::Valid | CredentialsStatus::ExpiringSoon)
    }

    pub(crate) fn refresh_token(&self) -> &str {
        &self.refresh_token
    }
}

impl TryFrom<&Session> for Credentials {
    type Error = String;

    fn try_from(session: &Session) -> Result<Self, Self::Error> {
        let access_token = session.access_token.clone();
        let refresh_token = session.refresh_token.clone();
        let expires_at = session.expires_at;
        Self::new(access_token, refresh_token, expires_at)
            .ok_or("Failed to create Credentials from invalid session data".to_string())
    }
}
