//! OAuth provider adapters

use crate::oauth::broker::ProviderConfig;

/// GitHub OAuth provider
pub fn github_provider(client_id: String) -> ProviderConfig {
    ProviderConfig {
        client_id,
        auth_url: "https://github.com/login/oauth/authorize".to_string(),
        token_url: "https://github.com/login/oauth/access_token".to_string(),
        device_auth_url: Some("https://github.com/login/device/code".to_string()),
        scopes: vec!["repo".to_string(), "read:user".to_string()],
    }
}

/// Google OAuth provider
pub fn google_provider(client_id: String) -> ProviderConfig {
    ProviderConfig {
        client_id,
        auth_url: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
        token_url: "https://oauth2.googleapis.com/token".to_string(),
        device_auth_url: Some("https://oauth2.googleapis.com/device/code".to_string()),
        scopes: vec!["openid".to_string(), "email".to_string()],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_github_provider() {
        let provider = github_provider("test-client-id".to_string());
        assert_eq!(provider.client_id, "test-client-id");
        assert!(provider.device_auth_url.is_some());
    }

    #[test]
    fn test_google_provider() {
        let provider = google_provider("test-client-id".to_string());
        assert_eq!(provider.client_id, "test-client-id");
        assert!(provider.device_auth_url.is_some());
    }
}
