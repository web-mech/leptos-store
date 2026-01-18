# Counter Example

A simple counter demonstrating the leptos-store `store!` macro with increment and decrement functionality.

## Features

- **Single Macro Definition**: Complete store with `store!` macro
- **State**: Auto-generated `CounterState` with defaults
- **Getters**: Read-only derived values with `this.read(|s| ...)`
- **Mutators**: State changes with `this.mutate(|s| ...)`
- **SSR Support**: Server-side rendering with Actix Web

## Running

```bash
# From the project root
make run NAME=counter-example

# Opens at http://127.0.0.1:3001
```

## The `store!` Macro

The `store!` macro generates a complete store implementation:

```rust
use leptos_store::store;

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
        }

        mutators {
            increment(this) {
                this.mutate(|s| s.count += 1);
            }

            decrement(this) {
                this.mutate(|s| s.count -= 1);
            }

            set_count(this, value: i32) {
                this.mutate(|s| s.count = value);
            }
        }
    }
}
```

### Key Points

- Use `this` (or any identifier) instead of `self` due to Rust 2024 macro hygiene
- **Getters**: Use `this.read(|s| ...)` for read-only access
- **Mutators**: Use `this.mutate(|s| ...)` for state changes
- The macro auto-implements the `Store` trait

## Generated API

The macro generates:

```rust
// State struct with Default
pub struct CounterState { pub count: i32 }

// Store struct
pub struct CounterStore { ... }

impl CounterStore {
    pub fn new() -> Self
    pub fn with_state(state: CounterState) -> Self

    // Getters
    pub fn doubled(&self) -> i32
    pub fn is_positive(&self) -> bool

    // Mutators
    pub fn increment(&self)
    pub fn decrement(&self)
    pub fn set_count(&self, value: i32)
}

impl Store for CounterStore { ... }
```

## Usage in Components

```rust
use leptos_store::prelude::*;
use crate::counter_store::CounterStore;

#[component]
fn Counter() -> impl IntoView {
    let store = use_store::<CounterStore>();

    view! {
        <div>
            <p>"Count: " {move || store.state().get().count}</p>
            <p>"Doubled: " {move || store.doubled()}</p>
            <button on:click=move |_| store.increment()>"+"</button>
            <button on:click=move |_| store.decrement()>"-"</button>
        </div>
    }
}
```
