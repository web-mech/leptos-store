# Counter Example

A simple counter demonstrating leptos-store basics with increment and decrement functionality.

## Features

- **State Management**: Uses `CounterStore` with reactive state
- **Getters**: Derived values like `doubled()`, `is_positive()`, `is_negative()`
- **Mutators**: State changes via `increment()`, `decrement()`, `reset()`
- **SSR Support**: Server-side rendering with Actix Web

## Running

```bash
# From the project root
make run NAME=counter-example

# Opens at http://127.0.0.1:3001
```

## Store Structure

```rust
impl CounterStore {
    // Getters - derived, read-only values
    pub fn doubled(&self) -> i32
    pub fn is_positive(&self) -> bool
    pub fn is_negative(&self) -> bool

    // Mutators - pure, synchronous state changes
    pub fn increment(&self)
    pub fn decrement(&self)
    pub fn reset(&self)
    pub fn set_count(&self, value: i32)
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
