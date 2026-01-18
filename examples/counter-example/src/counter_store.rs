// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 web-mech

//! Counter Store
//!
//! This module demonstrates the `store!` macro for defining
//! a complete store with state, getters, and mutators.

use leptos_store::store;

// Define the complete counter store using the store! macro
// Note: Use `this` (or any identifier) instead of `self` in bodies
// - Getters use this.read(|s| ...) for reads
// - Mutators use this.mutate(|s| ...) for writes
store! {
    pub CounterStore {
        state CounterState {
            count: i32 = 0,
        }

        getters {
            doubled(this) -> i32 {
                this.read(|s| s.count * 2)
            }

            is_positive(this) -> bool {
                this.read(|s| s.count > 0)
            }

            is_negative(this) -> bool {
                this.read(|s| s.count < 0)
            }

            is_prime(this) -> bool {
                this.read(|s| s.count > 1 && (2..=s.count-1).all(|i| s.count % i != 0))
            }
        }

        mutators {
            increment(this) {
                this.mutate(|s| s.count += 1);
            }

            decrement(this) {
                this.mutate(|s| s.count -= 1);
            }

            reset(this) {
                this.mutate(|s| s.count = 0);
            }

            set_count(this, value: i32) {
                this.mutate(|s| s.count = value);
            }
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::Get;
    use leptos_store::store::Store;

    #[test]
    fn test_counter_state_default() {
        let state = CounterState::default();
        assert_eq!(state.count, 0);
    }

    #[test]
    fn test_counter_store_creation() {
        let store = CounterStore::new();
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
