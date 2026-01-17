//! Auth Store Example
//!
//! This example demonstrates how to build a complete authentication
//! flow using leptos-store. It showcases:
//!
//! - Store definition with state, getters, and mutators
//! - Async actions for login/logout
//! - Context-based store sharing
//! - Reactive UI updates

pub mod auth_store;
pub mod components;

pub use auth_store::*;
pub use components::*;
