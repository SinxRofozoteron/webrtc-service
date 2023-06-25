use serde::Deserialize;

#[derive(Deserialize)]
pub struct OAuthCallbackQuery {
    pub state: String,
    pub code: String,
    pub scope: String,
}
