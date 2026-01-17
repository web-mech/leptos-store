//! # leptos-store
//!
//! Enterprise-grade, type-enforced state management for Leptos.
//!
//! This crate provides a structured, SSR-safe state management architecture
//! inspired by Vuex and Pinia, translated into idiomatic Rust for Leptos.
//!
//! ## Core Concepts
//!
//! Each store is a **domain module** composed of:
//!
//! 1. **State** - Read-only externally, reactive data container
//! 2. **Getters** - Derived, read-only computed values
//! 3. **Mutators** - Pure, synchronous state mutations
//! 4. **Actions** - Synchronous orchestration with side effects
//! 5. **Async Actions** - Asynchronous orchestration
//!
//! ## Mutation Rules
//!
//! | Layer | Can Write State | Async | Side Effects |
//! |-------|-----------------|-------|--------------|
//! | Components | ❌ | ❌ | ❌ |
//! | Getters | ❌ | ❌ | ❌ |
//! | Mutators | ✅ | ❌ | ❌ |
//! | Actions | ❌ | ❌ | ✅ |
//! | Async Actions | ❌ | ✅ | ✅ |
//!
//! **Only mutators may write state.**
//!
//! ## Available Macros
//!
//! | Macro | Purpose |
//! |-------|---------|
//! | [`define_state!`] | Define state structs with default values |
//! | [`define_action!`] | Define synchronous action structs |
//! | [`define_async_action!`] | Define async action structs with error types |
//! | [`impl_store!`] | Implement Store trait for an existing type |
//! | [`store!`] | Complete store definition in one macro |
//!
//! See the [`macros`] module for detailed documentation and examples.
//!
//! ## Example
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_store::prelude::*;
//!
//! // Define your state
//! #[derive(Clone, Debug, Default)]
//! pub struct CounterState {
//!     pub count: i32,
//! }
//!
//! // Define your store
//! #[derive(Clone)]
//! pub struct CounterStore {
//!     state: RwSignal<CounterState>,
//! }
//!
//! impl Store for CounterStore {
//!     type State = CounterState;
//!
//!     fn state(&self) -> ReadSignal<Self::State> {
//!         self.state.read_only()
//!     }
//! }
//!
//! // Define mutators
//! impl CounterStore {
//!     pub fn increment(&self) {
//!         self.state.update(|s| s.count += 1);
//!     }
//!
//!     pub fn decrement(&self) {
//!         self.state.update(|s| s.count -= 1);
//!     }
//! }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(unsafe_code)]

pub mod r#async;
pub mod context;
pub mod macros;
pub mod store;

pub mod prelude;

pub use prelude::*;
