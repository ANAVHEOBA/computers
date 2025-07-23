use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, ClientSecret, EndpointNotSet,
    EndpointSet, RedirectUrl, TokenResponse, TokenUrl,
};
use reqwest;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct GoogleUserInfo {
    pub email: String,
    pub given_name: String, // First name
    pub family_name: String, // Last name
    pub picture: Option<String>,
}

pub struct GoogleOauthService {
    client: BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>,
}

impl GoogleOauthService {
    pub fn new() -> Self {
        let google_client_id = ClientId::new(
            env::var("GOOGLE_CLIENT_ID").expect("Missing the GOOGLE_CLIENT_ID environment variable."),
        );
        let google_client_secret = ClientSecret::new(
            env::var("GOOGLE_CLIENT_SECRET").expect("Missing the GOOGLE_CLIENT_SECRET environment variable."),
        );
        let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
            .expect("Invalid authorization endpoint URL");
        let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
            .expect("Invalid token endpoint URL");

        // Set up the config for the Google OAuth2 process.
        let client = BasicClient::new(google_client_id)
            .set_client_secret(google_client_secret)
            .set_auth_uri(auth_url)
            .set_token_uri(token_url)
        .set_redirect_uri(
            RedirectUrl::new(env::var("GOOGLE_REDIRECT_URI").expect("Missing GOOGLE_REDIRECT_URI"))
                .expect("Invalid redirect URL"),
        );

        Self { client }
    }

    pub async fn exchange_code_for_user_info(
        &self,
        code: &str,
    ) -> Result<GoogleUserInfo, String> {
        // Create a reqwest client that doesn't follow redirects (for security)
        let http_client = reqwest::Client::new();

        // Exchange the code for a token.
        let token_result = self
            .client
            .exchange_code(AuthorizationCode::new(code.to_string()))
            .request_async(&http_client)
            .await
            .map_err(|e| format!("Failed to exchange token: {}", e))?;

        // Get user info directly from Google's userinfo endpoint
        let user_info_url = "https://www.googleapis.com/oauth2/v3/userinfo";
        let user_info_response = reqwest::Client::new()
            .get(user_info_url)
            .bearer_auth(token_result.access_token().secret())
            .send()
            .await
            .map_err(|e| format!("Failed to get user info: {}", e))?;

        if !user_info_response.status().is_success() {
            return Err(format!(
                "Failed to get user info, status: {}",
                user_info_response.status()
            ));
        }

        user_info_response
            .json::<GoogleUserInfo>()
            .await
            .map_err(|e| format!("Failed to parse user info: {}", e))
    }
} 