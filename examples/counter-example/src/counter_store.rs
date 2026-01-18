// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 web-mech

//! Counter Store
//!
//! This module demonstrates a simple counter store with
//! increment and decrement actions.

use leptos::prelude::*;
use leptos_store::prelude::*;

// ============================================================================
// State
// ============================================================================

/// Counter state - holds the current count value.
#[derive(Clone, Debug, Default)]
pub struct CounterState {
    pub count: i32,
}

// ============================================================================
// Store
// ============================================================================

/// A simple counter store demonstrating leptos-store basics.
///
/// This store provides:
/// - Getters for derived values (doubled, is_positive, is_negative)
/// - Mutators for state changes (increment, decrement, reset, set_count)
#[derive(Clone)]
pub struct CounterStore {
    state: RwSignal<CounterState>,
}

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
    // ========================================================================

    /// Get the current count doubled.
    pub fn doubled(&self) -> i32 {
        self.state.with(|s| s.count * 2)
    }

    /// Check if count is positive.
    pub fn is_positive(&self) -> bool {
        self.state.with(|s| s.count > 0)
    }

    /// Check if count is negative.
    pub fn is_negative(&self) -> bool {
        self.state.with(|s| s.count < 0)
    }

    // ========================================================================
    // Mutators - Pure, synchronous state changes
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

impl Store for CounterStore {
    type State = CounterState;

    fn state(&self) -> ReadSignal<Self::State> {
        self.state.read_only()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_store_creation() {
        let store = CounterStore::new();
        assert_eq!(store.state.get().count, 0);
    }

    #[test]
    fn test_increment() {
        let store = CounterStore::new();
        store.increment();
        assert_eq!(store.state.get().count, 1);
    }

    #[test]
    fn test_decrement() {
        let store = CounterStore::new();
        store.decrement();
        assert_eq!(store.state.get().count, -1);
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
        assert_eq!(store.state.get().count, 0);
    }
}
