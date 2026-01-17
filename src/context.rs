//! Context management for stores.
//!
//! This module provides utilities for integrating stores with Leptos'
//! context system, enabling stores to be shared across component trees.
//!
//! # Example
//!
//! ```rust,no_run
//! use leptos::prelude::*;
//! use leptos_store::prelude::*;
//!
//! #[derive(Clone, Debug, Default)]
//! struct MyState { name: String }
//!
//! #[derive(Clone)]
//! struct MyStore { state: RwSignal<MyState> }
//!
//! impl MyStore {
//!     fn new() -> Self {
//!         Self { state: RwSignal::new(MyState::default()) }
//!     }
//! }
//!
//! impl Store for MyStore {
//!     type State = MyState;
//!     fn state(&self) -> ReadSignal<Self::State> {
//!         self.state.read_only()
//!     }
//! }
//!
//! #[component]
//! pub fn App() -> impl IntoView {
//!     // Provide the store to all descendants
//!     provide_store(MyStore::new());
//!
//!     view! {
//!         <p>"Hello"</p>
//!     }
//! }
//! ```

use crate::store::{Store, StoreError};
use leptos::prelude::*;
use std::marker::PhantomData;

/// Provide a store to the component tree via Leptos context.
///
/// This function wraps the store in a way that makes it accessible
/// to all descendant components via `use_store`.
///
/// # Type Parameters
///
/// - `S`: The store type to provide. Must implement [`Store`].
///
/// # Example
///
/// ```rust,no_run
/// use leptos::prelude::*;
/// use leptos_store::prelude::*;
///
/// #[derive(Clone, Default)]
/// struct CounterState { count: i32 }
///
/// #[derive(Clone)]
/// struct CounterStore { state: RwSignal<CounterState> }
///
/// impl CounterStore {
///     fn new() -> Self { Self { state: RwSignal::new(CounterState::default()) } }
/// }
///
/// impl Store for CounterStore {
///     type State = CounterState;
///     fn state(&self) -> ReadSignal<Self::State> { self.state.read_only() }
/// }
///
/// #[component]
/// pub fn App() -> impl IntoView {
///     let store = CounterStore::new();
///     provide_store(store);
///     view! { <p>"Counter"</p> }
/// }
/// ```
pub fn provide_store<S: Store + Clone + Send + Sync + 'static>(store: S) {
    provide_context(StoreProvider::new(store));
}

/// Access a store from the Leptos context.
///
/// This function retrieves a store that was previously provided
/// via `provide_store`. It returns a clone of the store.
///
/// # Type Parameters
///
/// - `S`: The store type to retrieve. Must implement [`Store`].
///
/// # Panics
///
/// Panics if the store was not provided in the component tree.
/// Use `try_use_store` for a non-panicking alternative.
///
/// # Example
///
/// ```rust,no_run
/// use leptos::prelude::*;
/// use leptos_store::prelude::*;
///
/// #[derive(Clone, Default)]
/// struct CounterState { count: i32 }
///
/// #[derive(Clone)]
/// struct CounterStore { state: RwSignal<CounterState> }
///
/// impl Store for CounterStore {
///     type State = CounterState;
///     fn state(&self) -> ReadSignal<Self::State> { self.state.read_only() }
/// }
///
/// #[component]
/// fn Counter() -> impl IntoView {
///     let store = use_store::<CounterStore>();
///     let count = move || store.state().get().count;
///     view! { <span>{count}</span> }
/// }
/// ```
pub fn use_store<S: Store + Clone + Send + Sync + 'static>() -> S {
    use_context::<StoreProvider<S>>()
        .expect("Store not found in context. Did you forget to call provide_store?")
        .get()
}

/// Try to access a store from the Leptos context.
///
/// This is a non-panicking alternative to `use_store`.
///
/// # Returns
///
/// - `Ok(store)` if the store was found
/// - `Err(StoreError::ContextNotAvailable)` if the store was not provided
///
/// # Example
///
/// ```rust,no_run
/// use leptos::prelude::*;
/// use leptos_store::prelude::*;
/// use leptos_store::context::try_use_store;
///
/// #[derive(Clone, Default)]
/// struct CounterState { count: i32 }
///
/// #[derive(Clone)]
/// struct CounterStore { state: RwSignal<CounterState> }
///
/// impl Store for CounterStore {
///     type State = CounterState;
///     fn state(&self) -> ReadSignal<Self::State> { self.state.read_only() }
/// }
///
/// #[component]
/// fn MaybeCounter() -> impl IntoView {
///     match try_use_store::<CounterStore>() {
///         Ok(store) => view! { <span>{move || store.state().get().count}</span> }.into_any(),
///         Err(_) => view! { <span>"No counter store"</span> }.into_any(),
///     }
/// }
/// ```
pub fn try_use_store<S: Store + Clone + Send + Sync + 'static>() -> Result<S, StoreError> {
    use_context::<StoreProvider<S>>()
        .map(|p| p.get())
        .ok_or_else(|| {
            StoreError::ContextNotAvailable(format!(
                "Store {} not found in context",
                std::any::type_name::<S>()
            ))
        })
}

/// Wrapper for stores in Leptos context.
///
/// This struct wraps a store for use in Leptos' context system.
/// It handles cloning and provides a clean API for store access.
#[derive(Clone)]
pub struct StoreProvider<S: Store> {
    store: S,
    _marker: PhantomData<S>,
}

impl<S: Store + Clone + Send + Sync> StoreProvider<S> {
    /// Create a new store provider.
    pub fn new(store: S) -> Self {
        Self {
            store,
            _marker: PhantomData,
        }
    }

    /// Get a clone of the stored store.
    pub fn get(&self) -> S {
        self.store.clone()
    }

    /// Get a reference to the store.
    pub fn as_ref(&self) -> &S {
        &self.store
    }
}

/// Extension trait for stores to integrate with context.
pub trait StoreContextExt: Store + Sized {
    /// Provide this store to the component tree.
    fn provide(self)
    where
        Self: Clone + 'static,
    {
        provide_store(self);
    }
}

impl<S: Store> StoreContextExt for S {}

/// A scoped store provider that can be used for nested store instances.
///
/// This is useful when you need multiple instances of the same store type
/// in different parts of your component tree.
#[derive(Clone)]
pub struct ScopedStoreProvider<S: Store, const ID: u64 = 0> {
    store: S,
    _marker: PhantomData<S>,
}

impl<S: Store + Clone + Send + Sync, const ID: u64> ScopedStoreProvider<S, ID> {
    /// Create a new scoped store provider.
    pub fn new(store: S) -> Self {
        Self {
            store,
            _marker: PhantomData,
        }
    }

    /// Provide this scoped store to the context.
    pub fn provide(self) {
        provide_context(self);
    }

    /// Get a clone of the stored store.
    pub fn get(&self) -> S {
        self.store.clone()
    }
}

/// Access a scoped store from context.
///
/// # Type Parameters
///
/// - `S`: The store type
/// - `ID`: The scope identifier (const generic)
pub fn use_scoped_store<S: Store + Clone + Send + Sync + 'static, const ID: u64>() -> S {
    use_context::<ScopedStoreProvider<S, ID>>()
        .expect("Scoped store not found in context")
        .get()
}

/// Provide a scoped store to the context.
pub fn provide_scoped_store<S: Store + Clone + Send + Sync + 'static, const ID: u64>(store: S) {
    provide_context(ScopedStoreProvider::<S, ID>::new(store));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::Store;

    #[derive(Clone, Debug, Default, PartialEq)]
    struct TestState {
        value: i32,
    }

    #[derive(Clone)]
    struct TestStore {
        state: RwSignal<TestState>,
    }

    impl TestStore {
        fn new(initial: i32) -> Self {
            Self {
                state: RwSignal::new(TestState { value: initial }),
            }
        }
    }

    impl Store for TestStore {
        type State = TestState;

        fn state(&self) -> ReadSignal<Self::State> {
            self.state.read_only()
        }
    }

    #[test]
    fn test_store_provider_creation() {
        let store = TestStore::new(42);
        let provider = StoreProvider::new(store);

        let retrieved = provider.get();
        assert_eq!(retrieved.state.get().value, 42);
    }

    #[test]
    fn test_store_provider_as_ref() {
        let store = TestStore::new(100);
        let provider = StoreProvider::new(store);

        assert_eq!(provider.as_ref().state.get().value, 100);
    }

    #[test]
    fn test_scoped_store_provider() {
        let store = TestStore::new(50);
        let scoped: ScopedStoreProvider<TestStore, 1> = ScopedStoreProvider::new(store);

        let retrieved = scoped.get();
        assert_eq!(retrieved.state.get().value, 50);
    }

    #[test]
    fn test_store_error_context_not_available() {
        let err = StoreError::ContextNotAvailable("TestStore not found".to_string());
        assert!(err.to_string().contains("not available"));
    }
}
