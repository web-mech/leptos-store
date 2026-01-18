// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 web-mech

//! Counter Store
//!
//! This module demonstrates leptos-store macros for defining
//! a simple counter store with increment and decrement actions.

use leptos::prelude::*;
use leptos_store::{define_state, impl_store, store::Store};

// ============================================================================
// State - Using define_state! macro
// ============================================================================

define_state! {
    /// Counter state - holds the current count value.
    #[derive(Clone, Debug, PartialEq)]
    pub struct CounterState {
        /// The current count value
        pub count: i32 = 0,
    }
}

// ============================================================================
// Store
// ============================================================================

/// A simple counter store demonstrating leptos-store basics.
///
/// This store provides:
/// - Getters for derived values (doubled, is_positive, is_negative)
/// - Mutators for state changes (increment, decrement, reset, set_count)
///
/// The store uses the read/write split pattern:
/// - `state()` returns a `ReadSignal` for reactive reads (via Store trait)
/// - Mutators use the internal `RwSignal` for writes
#[derive(Clone)]
pub struct CounterStore {
    state: RwSignal<CounterState>,
}

// Implement Store trait using the impl_store! macro
// This enforces the read/write split by exposing only ReadSignal
impl_store!(CounterStore, CounterState, state);

impl Default for CounterStore {
    fn default() -> Self {
        Self::new()
    }
}

impl CounterStore {
    /// Create a new counter store with initial count of 0.
    pub fn new() -> Self {
        Self {
            state: RwSignal::new(CounterState::default()),
        }
    }

    /// Create a store with custom initial state.
    pub fn with_state(state: CounterState) -> Self {
        Self {
            state: RwSignal::new(state),
        }
    }

    // ========================================================================
    // Getters - Derived, read-only values
    // Uses self.state() which returns ReadSignal (from Store trait)
    // ========================================================================

    /// Get the current count doubled.
    pub fn doubled(&self) -> i32 {
        self.state().with(|s| s.count * 2)
    }

    /// Check if count is positive.
    pub fn is_positive(&self) -> bool {
        self.state().with(|s| s.count > 0)
    }

    /// Check if count is negative.
    pub fn is_negative(&self) -> bool {
        self.state().with(|s| s.count < 0)
    }

    // ========================================================================
    // Mutators - Pure, synchronous state changes
    // Uses self.state (RwSignal) directly for writes
    // ========================================================================

    /// Increment the counter by 1.
    pub fn increment(&self) {
        self.state.update(|s| s.count += 1);
    }

    /// Decrement the counter by 1.
    pub fn decrement(&self) {
        self.state.update(|s| s.count -= 1);
    }

    /// Reset the counter to zero.
    pub fn reset(&self) {
        self.state.update(|s| s.count = 0);
    }

    /// Set the counter to a specific value.
    pub fn set_count(&self, value: i32) {
        self.state.update(|s| s.count = value);
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use leptos_store::store::Store;

    #[test]
    fn test_counter_state_default() {
        // Test that define_state! generates correct defaults
        let state = CounterState::default();
        assert_eq!(state.count, 0);
    }

    #[test]
    fn test_counter_store_creation() {
        let store = CounterStore::new();
        // Use Store trait's state() method - returns ReadSignal
        assert_eq!(store.state().get().count, 0);
    }

    #[test]
    fn test_increment() {
        let store = CounterStore::new();
        store.increment();
        assert_eq!(store.state().get().count, 1);
    }

    #[test]
    fn test_decrement() {
        let store = CounterStore::new();
        store.decrement();
        assert_eq!(store.state().get().count, -1);
    }

    #[test]
    fn test_doubled() {
        let store = CounterStore::new();
        store.set_count(5);
        assert_eq!(store.doubled(), 10);
    }

    #[test]
    fn test_is_positive_negative() {
        let store = CounterStore::new();

        assert!(!store.is_positive());
        assert!(!store.is_negative());

        store.increment();
        assert!(store.is_positive());
        assert!(!store.is_negative());

        store.set_count(-5);
        assert!(!store.is_positive());
        assert!(store.is_negative());
    }

    #[test]
    fn test_reset() {
        let store = CounterStore::new();
        store.set_count(100);
        store.reset();
        assert_eq!(store.state().get().count, 0);
    }

    #[test]
    fn test_with_state() {
        let state = CounterState { count: 42 };
        let store = CounterStore::with_state(state);
        assert_eq!(store.state().get().count, 42);
    }
}
