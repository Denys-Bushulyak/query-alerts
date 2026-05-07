//! Crate-level re-exports.
//!
//! The top-level modules are:
//! - [`data_providers`] — HTTP fetching of alerts and query terms.
//! - [`dtos`] — deserialization DTOs and validation.
//! - [`entities`] — domain model types.
//! - [`etc`] — URL construction helpers.
//! - [`query`] — core matching algorithm (re-exported publicly).

pub mod algorithms;
pub mod data_providers;
pub mod dtos;
pub mod entities;
pub mod etc;
mod query;

/// Re-export of the main matching function.
pub use query::query;
pub use regex;
