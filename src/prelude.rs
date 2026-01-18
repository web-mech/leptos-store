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
//!
//! ## Hydration Support
//!
//! When the `hydrate` feature is enabled, additional types are available:
//!
//! ```rust,ignore
//! use leptos_store::prelude::*;
//!
//! // HydratableStore trait, provide_hydrated_store, use_hydrated_store, etc.
//! let store = use_hydrated_store::<MyStore>();
//! ```

// Core store traits and types
pub use crate::store::{
    Getter, Mutator, MutatorContext, ReadonlyStore, Store, StoreBuilder, StoreError, StoreId,
    StoreRegistry,
};

// Context management
pub use crate::context::{StoreProvider, provide_store, use_store};

// Async actions
pub use crate::r#async::{
    Action, ActionError, ActionFuture, ActionResult, ActionState, AsyncAction, AsyncActionBuilder,
};

// Hydration support (when feature is enabled)
#[cfg(feature = "hydrate")]
pub use crate::hydration::{
    HYDRATION_SCRIPT_PREFIX, HydratableStore, HydrationBuilder, StoreHydrationError,
    has_hydration_data, hydrate_store, hydration_script_html, hydration_script_id,
    serialize_store_state,
};

#[cfg(feature = "hydrate")]
pub use crate::context::{
    HydratableStoreContextExt, provide_hydrated_store, try_use_hydrated_store, use_hydrated_store,
};

// Re-export commonly used Leptos types for convenience
pub use leptos::prelude::{RwSignal, signal};

// Re-export serde when hydrate feature is enabled (for user convenience)
#[cfg(feature = "hydrate")]
pub use serde::{Deserialize, Serialize};
