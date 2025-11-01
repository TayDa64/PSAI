//! OAuth broker and authentication (Phase 3)

pub mod broker;
pub mod consent;
pub mod providers;
pub mod vault;

pub use broker::{OAuthBroker, ProviderConfig, TokenHandle};
pub use consent::ConsentLedger;
pub use providers::{github_provider, google_provider};
pub use vault::TokenVault;
