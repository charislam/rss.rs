#[derive(Clone, Debug)]
pub(crate) struct Credentials {
    access_token: String,
    refresh_token: String
}

impl Credentials {
    pub(crate) fn new(access_token: String, refresh_token: String) -> Self {
        Self {
            access_token,
            refresh_token
        }
    }
}