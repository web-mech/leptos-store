# Counter Example

A simple counter demonstrating leptos-store macros with increment and decrement functionality.

## Features

- **State Management**: Uses `define_state!` macro for state definition
- **Store Trait**: Uses `impl_store!` macro for read/write split pattern
- **Getters**: Derived values using `self.state()` (ReadSignal)
- **Mutators**: State changes using `self.state.update()` (RwSignal)
- **SSR Support**: Server-side rendering with Actix Web

## Running

```bash
# From the project root
make run NAME=counter-example

# Opens at http://127.0.0.1:3001
```

## Macros Used

### `define_state!` - State Definition

```rust
use leptos_store::define_state;

define_state! {
    #[derive(Clone, Debug, PartialEq)]
    pub struct CounterState {
        pub count: i32 = 0,
    }
}
```

### `impl_store!` - Store Trait Implementation

```rust
use leptos_store::impl_store;

#[derive(Clone)]
pub struct CounterStore {
    state: RwSignal<CounterState>,
}

// Implements Store trait, exposing state() -> ReadSignal
impl_store!(CounterStore, CounterState, state);
```

## Read/Write Split Pattern

The macros enforce a clean read/write split:

- **Getters** use `self.state()` which returns a `ReadSignal` (from Store trait)
- **Mutators** use `self.state.update()` which accesses the `RwSignal` directly

```rust
impl CounterStore {
    // Getter - uses ReadSignal from Store trait
    pub fn doubled(&self) -> i32 {
        self.state().with(|s| s.count * 2)
    }

    // Mutator - uses RwSignal directly
    pub fn increment(&self) {
        self.state.update(|s| s.count += 1);
    }
}
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
            <button on:click=move |_| store.increment()>"+"</button>
            <button on:click=move |_| store.decrement()>"-"</button>
        </div>
    }
}
```
