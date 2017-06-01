//! Guild Wars 2 web service bridge.

// Enums
mod version;
mod lang;

pub use self::version::Version;
pub use self::lang::Lang;

// Client
mod requester;
mod client;
mod builder;

pub use self::client::Client;
pub use self::builder::Builder;

// Models
mod models;
pub use self::models::*;