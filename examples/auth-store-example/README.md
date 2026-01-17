# Auth Store Example

This example demonstrates how to build a complete authentication flow using `leptos-store`.

## Features

- User authentication state management
- Login form with validation
- Dashboard with user information
- Logout functionality
- Reactive UI updates

## Running the Example

### Prerequisites

1. Install Rust and Cargo
2. Install trunk: `cargo install trunk`
3. Add the WebAssembly target: `rustup target add wasm32-unknown-unknown`

### Development

```bash
# From the examples/auth-store-example directory
trunk serve
```

Open http://localhost:8080 in your browser.

### Building for Production

```bash
trunk build --release
```

## Structure

```
src/
├── lib.rs           # Library entry point
├── auth_store.rs    # Authentication store implementation
└── components.rs    # Leptos UI components
```

## Store Architecture

The `AuthStore` follows the leptos-store conventions:

### State

```rust
pub struct AuthState {
    pub user: Option<User>,
    pub token: Option<AuthToken>,
    pub loading: bool,
    pub error: Option<AuthError>,
    pub remember_me: bool,
}
```

### Getters (Derived State)

- `is_authenticated()` - Check if user is logged in
- `current_user()` - Get the current user
- `display_name()` - Get user's name or "Guest"
- `user_initials()` - Get initials for avatar

### Mutators (State Changes)

- `set_user()` - Set the current user
- `set_token()` - Set the auth token
- `set_authenticated()` - Set both user and token
- `clear_auth()` - Clear all auth state (logout)

### Actions (Side Effects)

- `login()` - Perform login with credentials
- `logout()` - Perform logout
- `restore_session()` - Restore session from storage

## Demo

The example includes a demo mode - any email/password combination will log you in. In a real application, you would:

1. Make API calls in async actions
2. Store tokens securely
3. Handle session persistence
4. Implement proper validation
