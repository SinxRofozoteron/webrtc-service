use serde::Deserialize;

#[derive(Deserialize)]
pub struct StartOAuthQuery {
    pub success_redirect_url: Option<String>,
    pub failure_redirect_url: String,
    pub new_user_redirect_url: String,
}
