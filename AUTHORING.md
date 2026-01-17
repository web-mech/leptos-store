# Authoring Guide

> Complete guide for developing, contributing to, and maintaining leptos-store

This document provides comprehensive instructions for working with the leptos-store codebase, from initial setup through publishing releases.

---

## Table of Contents

- [Prerequisites](#prerequisites)
- [Getting Started](#getting-started)
- [Project Structure](#project-structure)
- [Development Workflow](#development-workflow)
- [Creating Stores](#creating-stores)
- [Testing](#testing)
- [Code Quality](#code-quality)
- [Documentation](#documentation)
- [Running the Example](#running-the-example)
- [Publishing](#publishing)
- [Architecture Reference](#architecture-reference)
- [Troubleshooting](#troubleshooting)

---

## Prerequisites

### Required Tools

| Tool | Minimum Version | Installation |
|------|-----------------|--------------|
| Rust | 1.85+ | [rustup.rs](https://rustup.rs) |
| Cargo | (bundled) | Comes with Rust |
| Make | 3.81+ | Usually pre-installed |

### Optional Tools

| Tool | Purpose | Installation |
|------|---------|--------------|
| trunk | WASM bundler for examples | `cargo install trunk` |
| cargo-watch | Auto-rebuild on changes | `cargo install cargo-watch` |
| cargo-tarpaulin | Code coverage | `cargo install cargo-tarpaulin` |
| cargo-audit | Security audits | `cargo install cargo-audit` |
| cargo-outdated | Dependency updates | `cargo install cargo-outdated` |

### WASM Target (for examples)

```bash
rustup target add wasm32-unknown-unknown
```

---

## Getting Started

### Clone and Setup

```bash
# Clone the repository
git clone https://github.com/your-org/leptos-store.git
cd leptos-store

# Verify setup
make check

# Run tests
make test
```

### Quick Commands Reference

```bash
make help          # Show all available commands
make check         # Verify compilation
make test          # Run all tests
make clippy        # Run lints
make fmt           # Format code
make example       # Run the auth example
make doc-open      # View documentation
```

---

## Project Structure

```
leptos-store/
├── Cargo.toml                 # Workspace configuration
├── Makefile                   # Build automation
├── README.md                  # User-facing documentation
├── AUTHORING.md               # This file
│
├── src/                       # Library source code
│   ├── lib.rs                 # Crate entry point, module docs
│   ├── prelude.rs             # Re-exports for users
│   ├── store.rs               # Core Store trait, builders
│   ├── context.rs             # Leptos context integration
│   ├── async.rs               # Async action support
│   └── macros.rs              # Declarative macros
│
├── examples/
│   └── auth-store-example/    # Complete auth example
│       ├── Cargo.toml
│       ├── index.html         # UI with styles
│       └── src/
│           ├── lib.rs
│           ├── auth_store.rs  # Store implementation
│           └── components.rs  # Leptos components
│
└── docs/
    └── specs/
        └── leptos-storekit-spec.md  # Original specification
```

### Module Responsibilities

| Module | Responsibility |
|--------|---------------|
| `store.rs` | Core `Store` trait, `Getter`, `Mutator`, `StoreBuilder`, `StoreRegistry` |
| `context.rs` | `provide_store`, `use_store`, `StoreProvider`, scoped stores |
| `async.rs` | `Action`, `AsyncAction`, `ReactiveAction`, `ActionState` |
| `macros.rs` | `store!`, `define_state!`, `define_action!`, `define_async_action!`, `impl_store!` |
| `prelude.rs` | Public API re-exports |

---

## Development Workflow

### Standard Development Cycle

```bash
# 1. Make changes to source files

# 2. Check compilation
make check

# 3. Run tests
make test

# 4. Check lints
make clippy

# 5. Format code
make fmt

# 6. Commit changes
git add -A && git commit -m "feat: your feature"
```

### Watch Mode (Auto-rebuild)

```bash
# Watch and run tests on changes
make watch

# Watch and run clippy on changes
make watch-clippy
```

### Feature Flags

The crate supports these feature flags:

| Flag | Description | Default |
|------|-------------|---------|
| `ssr` | Server-side rendering support | ✅ |
| `hydrate` | Hydration support | ❌ |
| `csr` | Client-side rendering only | ❌ |

Test all feature combinations:

```bash
make check-features
```

---

## Creating Stores

### Manual Store Definition

```rust
use leptos::prelude::*;
use leptos_store::prelude::*;

// 1. Define your state
#[derive(Clone, Debug, Default)]
pub struct CounterState {
    pub count: i32,
    pub name: String,
}

// 2. Define your store
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

    // Getters - derived, read-only
    pub fn doubled(&self) -> i32 {
        self.state.with(|s| s.count * 2)
    }

    // Mutators - pure state changes
    pub fn increment(&self) {
        self.state.update(|s| s.count += 1);
    }

    pub fn set_name(&self, name: String) {
        self.state.update(|s| s.name = name);
    }
}

// 3. Implement the Store trait
impl Store for CounterStore {
    type State = CounterState;

    fn state(&self) -> ReadSignal<Self::State> {
        self.state.read_only()
    }
}
```

### Using the `store!` Macro

```rust
use leptos_store::store;

store! {
    pub CounterStore {
        state CounterState {
            count: i32 = 0,
            name: String = "Counter".to_string(),
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
            set_name(name: String) {
                self.state.update(|s| s.name = name);
            }
        }
    }
}
```

### Using in Components

```rust
use leptos::prelude::*;
use leptos_store::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    // Provide store to component tree
    provide_store(CounterStore::new());

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
        </div>
    }
}
```

### Mutation Rules

| Layer | Can Write State | Async | Side Effects |
|-------|-----------------|-------|--------------|
| Components | ❌ | ❌ | ❌ |
| Getters | ❌ | ❌ | ❌ |
| Mutators | ✅ | ❌ | ❌ |
| Actions | ❌ | ❌ | ✅ |
| Async Actions | ❌ | ✅ | ✅ |

**Key principle: Only mutators may write state.**

---

## Testing

### Running Tests

```bash
# All tests
make test

# Verbose output
make test-verbose

# Library tests only
make test-lib

# Example tests only
make test-example

# With coverage
make coverage
```

### Writing Tests

Tests should be placed in a `#[cfg(test)]` module at the bottom of each file:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_creation() {
        let store = MyStore::new();
        assert_eq!(store.state().get().field, expected_value);
    }

    #[test]
    fn test_mutator() {
        let store = MyStore::new();
        store.set_field("new value".to_string());
        assert_eq!(store.state().get().field, "new value");
    }
}
```

### Test Organization

| Test Type | Location | Command |
|-----------|----------|---------|
| Unit tests | `src/*.rs` | `make test-lib` |
| Doc tests | Doc comments | `make test-doc` |
| Integration tests | `tests/*.rs` | `make test` |
| Example tests | `examples/*/src/*.rs` | `make test-example` |

---

## Code Quality

### Linting

```bash
# Run clippy
make clippy

# Auto-fix issues
make clippy-fix
```

### Formatting

```bash
# Format code
make fmt

# Check formatting (CI)
make fmt-check
```

### Full CI Pipeline

```bash
make ci  # Runs: fmt-check → clippy → test-all
```

### Code Style Guidelines

1. **Documentation**: All public items must have doc comments
2. **Error handling**: Use `thiserror` for custom errors
3. **Traits**: Prefer trait bounds over concrete types
4. **Naming**: Follow Rust conventions (snake_case, PascalCase)
5. **Imports**: Group by std → external → crate

---

## Documentation

### Building Docs

```bash
# Build documentation
make doc

# Build and open in browser
make doc-open

# Include private items
make doc-private
```

### Writing Documentation

Use rustdoc conventions:

```rust
/// Brief description of the item.
///
/// Longer description with more details about behavior,
/// edge cases, and usage patterns.
///
/// # Arguments
///
/// * `param` - Description of the parameter
///
/// # Returns
///
/// Description of what is returned.
///
/// # Panics
///
/// Conditions under which this function panics.
///
/// # Examples
///
/// ```rust,ignore
/// let result = my_function(arg);
/// assert_eq!(result, expected);
/// ```
pub fn my_function(param: Type) -> ReturnType {
    // ...
}
```

---

## Running the Example

### Development Mode

```bash
# Start with hot-reload (default port 8080)
make example

# Custom port
make example-port PORT=3000
```

Then open http://localhost:8080 in your browser.

### Production Build

```bash
# Debug build
make example-build

# Optimized release build
make example-release
```

Output will be in `examples/auth-store-example/dist/`.

### Example Features

The auth-store-example demonstrates:

- ✅ Store definition with state, getters, mutators
- ✅ Context-based store sharing
- ✅ Login form with validation
- ✅ Reactive UI updates
- ✅ Error handling
- ✅ Loading states

---

## Publishing

### Pre-release Checklist

- [ ] All tests pass: `make test-all`
- [ ] No clippy warnings: `make clippy`
- [ ] Code formatted: `make fmt-check`
- [ ] Documentation builds: `make doc`
- [ ] Version bumped in `Cargo.toml`
- [ ] CHANGELOG updated
- [ ] All changes committed

### Version Bumping

Edit `Cargo.toml`:

```toml
[package]
version = "0.2.0"  # Update this
```

### Publishing Steps

```bash
# 1. Verify everything works
make ci

# 2. Dry run (no actual publish)
make publish-dry

# 3. Create git tag
make tag

# 4. Publish to crates.io
make publish

# 5. Push tags
make tag-push
```

### Versioning Guidelines

Follow [Semantic Versioning](https://semver.org/):

- **MAJOR** (1.0.0): Breaking API changes
- **MINOR** (0.1.0): New features, backward compatible
- **PATCH** (0.0.1): Bug fixes, backward compatible

---

## Architecture Reference

### Core Abstractions

```
┌─────────────────────────────────────────────────────────────┐
│                        Component                             │
│  ┌─────────────────────────────────────────────────────┐    │
│  │  use_store::<MyStore>()                             │    │
│  │     │                                               │    │
│  │     ▼                                               │    │
│  │  ┌──────────────────────────────────────────────┐  │    │
│  │  │                   Store                       │  │    │
│  │  │  ┌────────────┐  ┌────────────┐             │  │    │
│  │  │  │   State    │  │  Getters   │ (read-only) │  │    │
│  │  │  │ RwSignal<T>│  │ derived    │             │  │    │
│  │  │  └────────────┘  └────────────┘             │  │    │
│  │  │         │                                    │  │    │
│  │  │         ▼                                    │  │    │
│  │  │  ┌────────────┐  ┌────────────┐             │  │    │
│  │  │  │  Mutators  │  │  Actions   │             │  │    │
│  │  │  │ state.update│  │ side effects│            │  │    │
│  │  │  └────────────┘  └────────────┘             │  │    │
│  │  └──────────────────────────────────────────────┘  │    │
│  └─────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

### Data Flow

```
User Event
    │
    ▼
Component Handler
    │
    ▼
Store Action (optional side effects)
    │
    ▼
Store Mutator (state.update)
    │
    ▼
Signal Update (RwSignal)
    │
    ▼
Reactive Update (automatic)
    │
    ▼
Component Re-render
```

### Type Hierarchy

```
Store (trait)
├── state() -> ReadSignal<State>
├── id() -> StoreId
└── name() -> &'static str

StoreProvider<S: Store>
├── new(store) -> Self
├── get() -> S
└── as_ref() -> &S

Getter<State, Output> (trait)
└── get(&self, state: &State) -> Output

Mutator<State> (trait)
└── mutate(&self, ctx: &mut MutatorContext<State>)

AsyncAction<S: Store> (trait)
├── Output (type)
├── Error (type)
└── execute(&self, store: &S) -> Future<ActionResult>
```

---

## Troubleshooting

### Common Issues

#### "Store not found in context"

**Cause**: `use_store` called before `provide_store`.

**Fix**: Ensure `provide_store` is called in a parent component:

```rust
#[component]
fn App() -> impl IntoView {
    provide_store(MyStore::new());  // Must be before children
    view! { <Child /> }
}
```

#### "cannot be sent between threads safely"

**Cause**: State type doesn't implement `Send + Sync`.

**Fix**: Ensure your state derives or implements the required traits:

```rust
#[derive(Clone, Debug, Default)]
pub struct MyState {
    // All fields must be Send + Sync
}
```

#### Trunk build fails

**Cause**: Missing WASM target.

**Fix**:

```bash
rustup target add wasm32-unknown-unknown
```

#### Clippy warnings about `Clone` on signals

**Cause**: Leptos signals are `Copy`, not just `Clone`.

**Fix**: Remove unnecessary `.clone()` calls on signals.

### Getting Help

1. Check the [README](./README.md) for basic usage
2. Read the [specification](./docs/specs/leptos-storekit-spec.md)
3. Run `make doc-open` to browse API documentation
4. Open an issue on GitHub

---

## Quick Reference Card

```bash
# Development
make check           # Check compilation
make test            # Run tests
make clippy          # Run lints
make fmt             # Format code
make ci              # Full CI pipeline

# Example
make example         # Run auth example
make example-release # Build optimized

# Documentation
make doc-open        # View docs in browser

# Publishing
make publish-dry     # Test publish
make publish         # Publish to crates.io

# Utilities
make clean           # Clean artifacts
make help            # Show all commands
make loc             # Lines of code stats
```

---

*Last updated: January 2026*
