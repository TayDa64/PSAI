//! OAuth broker for device code and PKCE flows

use anyhow::Result;
use oauth2::{
    AuthUrl, ClientId, DeviceAuthorizationUrl, Scope, TokenUrl,
    basic::BasicClient,
    reqwest::async_http_client,
    DeviceAuthorizationResponse,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::oauth::vault::TokenVault;

/// OAuth provider configuration
#[derive(Debug, Clone)]
pub struct ProviderConfig {
    pub client_id: String,
    pub auth_url: String,
    pub token_url: String,
    pub device_auth_url: Option<String>,
    pub scopes: Vec<String>,
}

/// OAuth token handle (not the actual token)
#[derive(Debug, Clone)]
pub struct TokenHandle {
    pub id: String,
    pub provider: String,
    pub scopes: Vec<String>,
}

/// OAuth broker
pub struct OAuthBroker {
    vault: Arc<TokenVault>,
    providers: Arc<RwLock<HashMap<String, ProviderConfig>>>,
}

impl OAuthBroker {
    pub fn new(vault: Arc<TokenVault>) -> Self {
        OAuthBroker {
            vault,
            providers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a provider
    pub async fn register_provider(&self, name: String, config: ProviderConfig) {
        let mut providers = self.providers.write().await;
        providers.insert(name, config);
    }

    /// Request a token via device code flow
    pub async fn request_token_device_code(
        &self,
        provider: &str,
        scopes: Vec<String>,
    ) -> Result<TokenHandle> {
        let providers = self.providers.read().await;
        let config = providers.get(provider)
            .ok_or_else(|| anyhow::anyhow!("Provider not found: {}", provider))?;

        tracing::info!("Starting device code flow for provider: {}", provider);

        // Create OAuth client
        let client = BasicClient::new(
            ClientId::new(config.client_id.clone()),
            None,
            AuthUrl::new(config.auth_url.clone())?,
            Some(TokenUrl::new(config.token_url.clone())?),
        );

        let client = if let Some(device_url) = &config.device_auth_url {
            client.set_device_authorization_url(DeviceAuthorizationUrl::new(device_url.clone())?)
        } else {
            client
        };

        // Request device authorization
        let device_auth = client
            .exchange_device_code()?
            .add_scopes(scopes.iter().map(|s| Scope::new(s.clone())))
            .request_async(async_http_client)
            .await?;

        // Display user code and verification URL
        tracing::info!("Device code: {}", device_auth.user_code().secret());
        tracing::info!("Verification URL: {}", device_auth.verification_uri());

        // In a real implementation:
        // 1. Display the code to the user in the TUI
        // 2. Poll for token
        // 3. Store token in vault
        // 4. Return handle

        let handle_id = uuid::Uuid::new_v4().to_string();
        let handle = TokenHandle {
            id: handle_id.clone(),
            provider: provider.to_string(),
            scopes,
        };

        // Store placeholder token in vault
        self.vault.store(&handle_id, "placeholder-token").await?;

        Ok(handle)
    }

    /// Request a token via PKCE flow
    pub async fn request_token_pkce(
        &self,
        provider: &str,
        scopes: Vec<String>,
    ) -> Result<TokenHandle> {
        tracing::info!("Starting PKCE flow for provider: {}", provider);
        
        // Placeholder for PKCE implementation
        // Real implementation would:
        // 1. Generate code verifier and challenge
        // 2. Redirect to authorization URL
        // 3. Handle callback
        // 4. Exchange code for token
        // 5. Store in vault
        
        let handle_id = uuid::Uuid::new_v4().to_string();
        let handle = TokenHandle {
            id: handle_id.clone(),
            provider: provider.to_string(),
            scopes,
        };

        self.vault.store(&handle_id, "placeholder-token").await?;

        Ok(handle)
    }

    /// Refresh a token
    pub async fn refresh(&self, handle: &TokenHandle) -> Result<()> {
        tracing::info!("Refreshing token for handle: {}", handle.id);
        
        // Real implementation would:
        // 1. Retrieve refresh token from vault
        // 2. Exchange for new access token
        // 3. Update vault
        
        Ok(())
    }

    /// Revoke a token
    pub async fn revoke(&self, handle: &TokenHandle) -> Result<()> {
        tracing::info!("Revoking token for handle: {}", handle.id);
        
        // Real implementation would:
        // 1. Call provider's revocation endpoint
        // 2. Remove from vault
        // 3. Log in consent ledger
        
        self.vault.delete(&handle.id).await?;
        
        Ok(())
    }

    /// Get token for a handle (used by broker, not exposed to agents)
    pub async fn get_token(&self, handle: &TokenHandle) -> Result<String> {
        self.vault.fetch(&handle.id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_broker_creation() {
        let vault = Arc::new(TokenVault::new_in_memory());
        let broker = OAuthBroker::new(vault);
        
        // Test provider registration
        let config = ProviderConfig {
            client_id: "test-client".to_string(),
            auth_url: "https://example.com/auth".to_string(),
            token_url: "https://example.com/token".to_string(),
            device_auth_url: None,
            scopes: vec!["read".to_string()],
        };
        
        broker.register_provider("test".to_string(), config).await;
    }
}
