//! Prelude module - re-exports all commonly used types and traits.
//!
//! Import this module to get started quickly:
//!
//! ```rust
//! use leptos_store::prelude::*;
//!
//! // Now you have access to Store, Getter, Mutator, etc.
//! let state = ActionState::Idle;
//! assert!(state.is_idle());
//! ```

// Core store traits and types
pub use crate::store::{
    Getter, Mutator, MutatorContext, ReadonlyStore, Store, StoreBuilder, StoreError, StoreId,
    StoreRegistry,
};

// Context management
pub use crate::context::{provide_store, use_store, StoreProvider};

// Async actions
pub use crate::r#async::{
    Action, ActionError, ActionFuture, ActionResult, ActionState, AsyncAction, AsyncActionBuilder,
};

// Re-export commonly used Leptos types for convenience
pub use leptos::prelude::{signal, RwSignal};
