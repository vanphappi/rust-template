use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    RedirectUrl, Scope, TokenResponse, TokenUrl,
    basic::BasicClient,
    reqwest::async_http_client,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::errors::ApiError;

/// OAuth2 provider configuration
#[derive(Debug, Clone)]
pub struct OAuth2Provider {
    pub name: String,
    pub client: BasicClient,
    pub scopes: Vec<String>,
}

/// OAuth2 configuration for multiple providers
#[derive(Debug, Clone)]
pub struct OAuth2Config {
    providers: HashMap<String, OAuth2Provider>,
}

/// OAuth2 user info from provider
#[derive(Debug, Serialize, Deserialize)]
pub struct OAuth2UserInfo {
    pub id: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub provider: String,
}

/// OAuth2 authorization URL response
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizationUrlResponse {
    pub auth_url: String,
    pub csrf_token: String,
    pub pkce_verifier: Option<String>,
}

impl OAuth2Config {
    /// Create new OAuth2 configuration
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    /// Add Google OAuth2 provider
    pub fn add_google(
        mut self,
        client_id: String,
        client_secret: String,
        redirect_url: String,
    ) -> Result<Self, ApiError> {
        let client = BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
                .map_err(|e| ApiError::configuration(format!("Invalid Google auth URL: {}", e)))?,
            Some(
                TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
                    .map_err(|e| ApiError::configuration(format!("Invalid Google token URL: {}", e)))?,
            ),
        )
        .set_redirect_uri(
            RedirectUrl::new(redirect_url)
                .map_err(|e| ApiError::configuration(format!("Invalid redirect URL: {}", e)))?,
        );

        self.providers.insert(
            "google".to_string(),
            OAuth2Provider {
                name: "google".to_string(),
                client,
                scopes: vec![
                    "openid".to_string(),
                    "email".to_string(),
                    "profile".to_string(),
                ],
            },
        );

        Ok(self)
    }

    /// Add GitHub OAuth2 provider
    pub fn add_github(
        mut self,
        client_id: String,
        client_secret: String,
        redirect_url: String,
    ) -> Result<Self, ApiError> {
        let client = BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
                .map_err(|e| ApiError::configuration(format!("Invalid GitHub auth URL: {}", e)))?,
            Some(
                TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
                    .map_err(|e| ApiError::configuration(format!("Invalid GitHub token URL: {}", e)))?,
            ),
        )
        .set_redirect_uri(
            RedirectUrl::new(redirect_url)
                .map_err(|e| ApiError::configuration(format!("Invalid redirect URL: {}", e)))?,
        );

        self.providers.insert(
            "github".to_string(),
            OAuth2Provider {
                name: "github".to_string(),
                client,
                scopes: vec!["user:email".to_string()],
            },
        );

        Ok(self)
    }

    /// Add Microsoft OAuth2 provider
    pub fn add_microsoft(
        mut self,
        client_id: String,
        client_secret: String,
        redirect_url: String,
        tenant_id: Option<String>,
    ) -> Result<Self, ApiError> {
        let tenant = tenant_id.unwrap_or_else(|| "common".to_string());
        
        let client = BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new(format!(
                "https://login.microsoftonline.com/{}/oauth2/v2.0/authorize",
                tenant
            ))
            .map_err(|e| ApiError::configuration(format!("Invalid Microsoft auth URL: {}", e)))?,
            Some(
                TokenUrl::new(format!(
                    "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
                    tenant
                ))
                .map_err(|e| ApiError::configuration(format!("Invalid Microsoft token URL: {}", e)))?,
            ),
        )
        .set_redirect_uri(
            RedirectUrl::new(redirect_url)
                .map_err(|e| ApiError::configuration(format!("Invalid redirect URL: {}", e)))?,
        );

        self.providers.insert(
            "microsoft".to_string(),
            OAuth2Provider {
                name: "microsoft".to_string(),
                client,
                scopes: vec![
                    "openid".to_string(),
                    "email".to_string(),
                    "profile".to_string(),
                ],
            },
        );

        Ok(self)
    }

    /// Get authorization URL for a provider
    pub fn get_authorization_url(
        &self,
        provider: &str,
        use_pkce: bool,
    ) -> Result<AuthorizationUrlResponse, ApiError> {
        let oauth_provider = self
            .providers
            .get(provider)
            .ok_or_else(|| ApiError::not_found_resource(
                format!("OAuth2 provider '{}' not found", provider),
                "oauth2_provider"
            ))?;

        let mut auth_request = oauth_provider.client.authorize_url(CsrfToken::new_random);

        // Add scopes
        for scope in &oauth_provider.scopes {
            auth_request = auth_request.add_scope(Scope::new(scope.clone()));
        }

        let (pkce_challenge, pkce_verifier) = if use_pkce {
            let (challenge, verifier) = PkceCodeChallenge::new_random_sha256();
            (Some(challenge), Some(verifier.secret().clone()))
        } else {
            (None, None)
        };

        let (auth_url, csrf_token) = if let Some(challenge) = pkce_challenge {
            auth_request.set_pkce_challenge(challenge).url()
        } else {
            auth_request.url()
        };

        Ok(AuthorizationUrlResponse {
            auth_url: auth_url.to_string(),
            csrf_token: csrf_token.secret().clone(),
            pkce_verifier,
        })
    }

    /// Exchange authorization code for access token
    pub async fn exchange_code(
        &self,
        provider: &str,
        code: String,
        _pkce_verifier: Option<String>,
    ) -> Result<String, ApiError> {
        let oauth_provider = self
            .providers
            .get(provider)
            .ok_or_else(|| ApiError::not_found_resource(
                format!("OAuth2 provider '{}' not found", provider),
                "oauth2_provider"
            ))?;

        let token_result = oauth_provider
            .client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(async_http_client)
            .await
            .map_err(|e| ApiError::external_service(
                format!("Failed to exchange code: {}", e),
                provider
            ))?;

        Ok(token_result.access_token().secret().clone())
    }

    /// Get user info from provider using access token
    pub async fn get_user_info(
        &self,
        provider: &str,
        access_token: &str,
    ) -> Result<OAuth2UserInfo, ApiError> {
        match provider {
            "google" => self.get_google_user_info(access_token).await,
            "github" => self.get_github_user_info(access_token).await,
            "microsoft" => self.get_microsoft_user_info(access_token).await,
            _ => Err(ApiError::not_found_resource(
                format!("OAuth2 provider '{}' not supported", provider),
                "oauth2_provider"
            )),
        }
    }

    /// Get Google user info
    async fn get_google_user_info(&self, access_token: &str) -> Result<OAuth2UserInfo, ApiError> {
        let client = reqwest::Client::new();
        let response = client
            .get("https://www.googleapis.com/oauth2/v2/userinfo")
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| ApiError::external_service(
                format!("Failed to get Google user info: {}", e),
                "google"
            ))?;

        #[derive(Deserialize)]
        struct GoogleUserInfo {
            id: String,
            email: Option<String>,
            name: Option<String>,
            picture: Option<String>,
        }

        let user_info: GoogleUserInfo = response
            .json()
            .await
            .map_err(|e| ApiError::external_service(
                format!("Failed to parse Google user info: {}", e),
                "google"
            ))?;

        Ok(OAuth2UserInfo {
            id: user_info.id,
            email: user_info.email,
            name: user_info.name,
            picture: user_info.picture,
            provider: "google".to_string(),
        })
    }

    /// Get GitHub user info
    async fn get_github_user_info(&self, access_token: &str) -> Result<OAuth2UserInfo, ApiError> {
        let client = reqwest::Client::new();

        // Get user profile
        let response = client
            .get("https://api.github.com/user")
            .bearer_auth(access_token)
            .header("User-Agent", "api-management-template")
            .send()
            .await
            .map_err(|e| ApiError::external_service(
                format!("Failed to get GitHub user info: {}", e),
                "github"
            ))?;

        #[derive(Deserialize)]
        struct GitHubUserInfo {
            id: u64,
            email: Option<String>,
            name: Option<String>,
            avatar_url: Option<String>,
        }

        let user_info: GitHubUserInfo = response
            .json()
            .await
            .map_err(|e| ApiError::external_service(
                format!("Failed to parse GitHub user info: {}", e),
                "github"
            ))?;

        Ok(OAuth2UserInfo {
            id: user_info.id.to_string(),
            email: user_info.email,
            name: user_info.name,
            picture: user_info.avatar_url,
            provider: "github".to_string(),
        })
    }
    /// Get Microsoft user info
    async fn get_microsoft_user_info(&self, access_token: &str) -> Result<OAuth2UserInfo, ApiError> {
        let client = reqwest::Client::new();
        let response = client
            .get("https://graph.microsoft.com/v1.0/me")
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| ApiError::external_service(
                format!("Failed to get Microsoft user info: {}", e),
                "microsoft"
            ))?;

        #[derive(Deserialize)]
        struct MicrosoftUserInfo {
            id: String,
            #[serde(rename = "userPrincipalName")]
            email: Option<String>,
            #[serde(rename = "displayName")]
            name: Option<String>,
        }

        let user_info: MicrosoftUserInfo = response
            .json()
            .await
            .map_err(|e| ApiError::external_service(
                format!("Failed to parse Microsoft user info: {}", e),
                "microsoft"
            ))?;

        Ok(OAuth2UserInfo {
            id: user_info.id,
            email: user_info.email,
            name: user_info.name,
            picture: None,
            provider: "microsoft".to_string(),
        })
    }

    /// Get provider by name
    pub fn get_provider(&self, name: &str) -> Option<&OAuth2Provider> {
        self.providers.get(name)
    }

    /// List all configured providers
    pub fn list_providers(&self) -> Vec<String> {
        self.providers.keys().cloned().collect()
    }
}

impl Default for OAuth2Config {
    fn default() -> Self {
        Self::new()
    }
}
