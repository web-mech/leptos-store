# Auth Store Example

A complete authentication example demonstrating `leptos-store` with SSR (Server-Side Rendering) support.

## Features

- ✅ Store definition with state, getters, and mutators
- ✅ Context-based store sharing
- ✅ Login form with validation
- ✅ Reactive UI updates
- ✅ Error handling
- ✅ Loading states
- ✅ SSR (Server-Side Rendering) with Actix Web
- ✅ Hydration support

## Prerequisites

```bash
# Install cargo-leptos (recommended for SSR)
cargo install cargo-leptos

# Or for manual builds, ensure WASM target is installed
rustup target add wasm32-unknown-unknown
```

## Running the Example

### With cargo-leptos (Recommended)

```bash
# Development mode with hot-reload
cargo leptos watch

# Production build
cargo leptos build --release
```

### Manual SSR Mode

```bash
# Build the WASM for hydration
cargo build --lib --features hydrate --target wasm32-unknown-unknown

# Run the server
cargo run --features ssr
```

Then open http://127.0.0.1:3000 in your browser.

### CSR Mode (Client-Side Only)

For a simpler setup without SSR:

```bash
# Install trunk
cargo install trunk

# Run with trunk
trunk serve --features csr
```

## Project Structure

```
auth-store-example/
├── Cargo.toml              # Dependencies and features
├── src/
│   ├── lib.rs              # Library entry (hydration)
│   ├── main.rs             # Server entry (SSR)
│   ├── auth_store.rs       # Store implementation
│   └── components.rs       # Leptos components
└── style/
    └── main.css            # Styles
```

## Features Explained

| Feature | Description |
|---------|-------------|
| `ssr` | Server-side rendering with Actix Web |
| `hydrate` | Client-side hydration of SSR HTML |
| `csr` | Client-side rendering only (no server) |

## Store Architecture

```
AuthStore
├── State: AuthState
│   ├── user: Option<User>
│   ├── is_loading: bool
│   └── error: Option<AuthError>
├── Getters
│   ├── is_authenticated() -> bool
│   ├── current_user() -> Option<User>
│   ├── user_email() -> Option<String>
│   └── display_name() -> String
└── Mutators
    ├── login(credentials)
    └── logout()
```

## License

MIT OR Apache-2.0
