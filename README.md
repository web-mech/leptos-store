# leptos-store

> Enterprise-grade, type-enforced state management for Leptos

[![Crates.io](https://img.shields.io/crates/v/leptos-store.svg)](https://crates.io/crates/leptos-store)
[![Documentation](https://docs.rs/leptos-store/badge.svg)](https://docs.rs/leptos-store)
[![License](https://img.shields.io/crates/l/leptos-store.svg)](LICENSE)

## Overview

`leptos-store` provides a structured, SSR-safe state management architecture for [Leptos](https://leptos.dev), inspired by **Vuex** and **Pinia**, translated into idiomatic Rust.

Leptos provides excellent primitives (signals, context, resources), but no canonical, scalable state architecture. This creates problems for large teams, enterprise governance, long-lived applications, SSR correctness, and auditing.

**leptos-store exists to solve structure, not reactivity.**

## Features

- 🏗️ **Global, namespaced stores** - Clear domain boundaries
- 🔒 **Predictable mutation flow** - Only mutators can write state
- 🌐 **First-class SSR support** - Works seamlessly with server-side rendering
- ⚡ **Async-safe actions** - Built-in support for async operations
- 🔧 **Compile-time enforcement** - Catch errors at compile time, not runtime
- 📦 **Zero magic** - No hidden executors or runtime reflection

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
leptos-store = "0.1"
leptos = "0.8"
```

## Quick Start

### Define Your Store

```rust
use leptos::prelude::*;
use leptos_store::prelude::*;

// Define your state
#[derive(Clone, Debug, Default)]
pub struct CounterState {
    pub count: i32,
}

// Define your store
#[derive(Clone)]
pub struct CounterStore {
    state: RwSignal<CounterState>,
}

impl CounterStore {
    pub fn new() -> Self {
        Self {
            state: RwSignal::new(CounterState::default()),
        }
    }

    // Getters - derived, read-only values
    pub fn doubled(&self) -> i32 {
        self.state.with(|s| s.count * 2)
    }

    // Mutators - pure, synchronous state changes
    pub fn increment(&self) {
        self.state.update(|s| s.count += 1);
    }

    pub fn decrement(&self) {
        self.state.update(|s| s.count -= 1);
    }

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
```

### Use in Components

```rust
#[component]
pub fn App() -> impl IntoView {
    // Provide store to component tree
    let store = CounterStore::new();
    provide_store(store);

    view! {
        <Counter />
    }
}

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

### Using the `store!` Macro

For less boilerplate, use the declarative macro:

```rust
use leptos_store::store;

store! {
    pub CounterStore {
        state CounterState {
            count: i32 = 0,
        }

        getters {
            doubled() -> i32 {
                self.state().with(|s| s.count * 2)
            }
        }

        mutators {
            increment() {
                self.state.update(|s| s.count += 1);
            }
            decrement() {
                self.state.update(|s| s.count -= 1);
            }
            set_count(value: i32) {
                self.state.update(|s| s.count = value);
            }
        }
    }
}
```

## Available Macros

| Macro | Purpose |
|-------|---------|
| `define_state!` | Define state structs with default values |
| `define_action!` | Define synchronous action structs |
| `define_async_action!` | Define async action structs with result types |
| `impl_store!` | Implement Store trait for an existing type |
| `store!` | Complete store definition in one macro |

### `define_state!` - State with Defaults

```rust
use leptos_store::define_state;

define_state! {
    #[derive(Clone, Debug, PartialEq)]
    pub struct UserState {
        name: String,                    // Uses String::default()
        email: Option<String>,           // Uses None
        age: u32 = 0,                    // Explicit default
        active: bool = true,             // Explicit default
    }
}

let user = UserState::default();
assert_eq!(user.name, "");
assert!(user.active);
```

### `define_action!` - Synchronous Actions

```rust
use leptos_store::define_action;

define_action! {
    /// Updates user profile information
    #[derive(Debug, Clone)]
    pub UpdateProfileAction {
        user_id: String,
        name: Option<String>,
        email: Option<String>,
    }
}

let action = UpdateProfileAction::new(
    "user_123".to_string(),
    Some("John Doe".to_string()),
    None,
);
```

### `define_async_action!` - Async Actions with Error Types

```rust
use leptos_store::define_async_action;

// Define your error type
#[derive(Debug, Clone)]
enum ApiError {
    Network(String),
    NotFound,
    Unauthorized,
}

// Define the async action
define_async_action! {
    /// Fetches user data from the API
    #[derive(Debug, Clone)]
    pub FetchUserAction {
        user_id: String,
        include_profile: bool,
    } -> Result<UserData, ApiError>
}

let action = FetchUserAction::new("user_123".to_string(), true);

// Helper methods for documentation
assert!(FetchUserAction::result_type_description().contains("Result"));
assert_eq!(FetchUserAction::output_type_name(), "UserData");
assert_eq!(FetchUserAction::error_type_name(), "ApiError");
```

### `impl_store!` - Quick Store Trait Implementation

```rust
use leptos::prelude::*;
use leptos_store::{impl_store, store::Store};

#[derive(Clone, Default)]
struct CartState {
    items: Vec<String>,
    total: f64,
}

#[derive(Clone)]
struct CartStore {
    state: RwSignal<CartState>,
}

// One-liner to implement Store trait
impl_store!(CartStore, CartState, state);
```

## Conceptual Model

Each store is a **domain module** composed of:

| Layer | Description | Can Write State | Async | Side Effects |
|-------|-------------|-----------------|-------|--------------|
| **State** | Read-only externally | N/A | ❌ | ❌ |
| **Getters** | Derived, read-only | ❌ | ❌ | ❌ |
| **Mutators** | Pure, synchronous writes | ✅ | ❌ | ❌ |
| **Actions** | Sync orchestration | ❌ | ❌ | ✅ |
| **Async Actions** | Async orchestration | ❌ | ✅ | ✅ |

**Only mutators may write state.** This is the core principle that ensures predictability.

## Advanced Usage

### Async Actions

```rust
use leptos_store::prelude::*;

pub struct LoginAction {
    pub email: String,
    pub password: String,
}

impl AsyncAction<AuthStore> for LoginAction {
    type Output = AuthToken;
    type Error = AuthError;

    async fn execute(&self, store: &AuthStore) -> ActionResult<Self::Output, Self::Error> {
        // Perform async operation
        let token = auth_api::login(&self.email, &self.password).await?;
        
        // Dispatch mutation
        store.set_authenticated(true, token.clone());
        
        Ok(token)
    }
}
```

### Scoped Stores

For multiple instances of the same store type:

```rust
// Provide scoped stores with unique IDs
provide_scoped_store::<CounterStore, 1>(counter1);
provide_scoped_store::<CounterStore, 2>(counter2);

// Access scoped stores
let counter1 = use_scoped_store::<CounterStore, 1>();
let counter2 = use_scoped_store::<CounterStore, 2>();
```

### Store Registry

For debugging and hot-reloading:

```rust
let mut registry = StoreRegistry::new();
registry.register(my_store)?;

// Later...
let store = registry.get::<MyStore>();
```

## Design Philosophy

### Convention over Primitives

Instead of giving you raw signals and hoping for the best, leptos-store provides a structured architecture that scales.

### Compile-time Enforcement

The type system prevents invalid state transitions. If it compiles, it follows the rules.

### SSR-First Design

Every feature is designed with server-side rendering in mind. No hydration mismatches.

## Examples

See the `examples/` directory for complete examples:

- `auth-store-example` - User authentication flow with login/logout

## Contributing

We welcome contributions! See [`AUTHORING.md`](./AUTHORING.md) for:

- Development setup and workflow
- Project structure and architecture
- Testing and code quality guidelines
- Publishing releases

```bash
# Quick start for contributors
git clone https://github.com/your-org/leptos-store.git
cd leptos-store
make check   # Verify setup
make test    # Run tests
make help    # See all commands
```

## License

MIT OR Apache-2.0
