//! OAuth broker and authentication (Phase 3)

pub mod broker;
pub mod providers;
pub mod vault;
pub mod consent;

pub use broker::{OAuthBroker, ProviderConfig, TokenHandle};
pub use vault::TokenVault;
pub use consent::ConsentLedger;
pub use providers::{github_provider, google_provider};
