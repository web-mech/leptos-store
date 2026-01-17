//! Authentication Store
//!
//! This module defines the authentication store with all its
//! state, getters, mutators, and actions.

use leptos::prelude::*;
use leptos_store::prelude::*;
use thiserror::Error;

// ============================================================================
// Types
// ============================================================================

/// Represents a logged-in user.
#[derive(Clone, Debug, PartialEq)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
    pub avatar_url: Option<String>,
}

/// Authentication token.
#[derive(Clone, Debug, PartialEq)]
pub struct AuthToken {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: u64,
}

/// Login credentials.
#[derive(Clone, Debug)]
pub struct LoginCredentials {
    pub email: String,
    pub password: String,
    pub remember_me: bool,
}

/// Registration data.
#[derive(Clone, Debug)]
pub struct RegistrationData {
    pub email: String,
    pub password: String,
    pub name: String,
}

// ============================================================================
// Errors
// ============================================================================

/// Authentication errors.
#[derive(Debug, Error, Clone, PartialEq)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("User not found")]
    UserNotFound,

    #[error("Email already exists")]
    EmailExists,

    #[error("Token expired")]
    TokenExpired,

    #[error("Network error: {0}")]
    Network(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

// ============================================================================
// State
// ============================================================================

/// Authentication state.
#[derive(Clone, Debug, Default)]
pub struct AuthState {
    /// Current user (None if not logged in).
    pub user: Option<User>,

    /// Authentication token.
    pub token: Option<AuthToken>,

    /// Whether authentication is in progress.
    pub loading: bool,

    /// Last error that occurred.
    pub error: Option<AuthError>,

    /// Whether "remember me" is enabled.
    pub remember_me: bool,
}

impl AuthState {
    /// Check if user is authenticated.
    pub fn is_authenticated(&self) -> bool {
        self.user.is_some() && self.token.is_some()
    }

    /// Check if token is expired.
    pub fn is_token_expired(&self) -> bool {
        self.token
            .as_ref()
            .map(|t| {
                // In a real app, compare with current timestamp
                t.expires_at == 0
            })
            .unwrap_or(true)
    }
}

// ============================================================================
// Store
// ============================================================================

/// Authentication store.
///
/// This store manages all authentication-related state including:
/// - Current user
/// - Authentication token
/// - Loading state
/// - Error state
///
/// # Example
///
/// ```rust,ignore
/// use auth_store_example::AuthStore;
/// use leptos_store::prelude::*;
///
/// let store = AuthStore::new();
/// provide_store(store);
///
/// // In a component
/// let store = use_store::<AuthStore>();
/// let is_logged_in = store.is_authenticated();
/// ```
#[derive(Clone)]
pub struct AuthStore {
    state: RwSignal<AuthState>,
}

impl Default for AuthStore {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthStore {
    /// Create a new authentication store.
    pub fn new() -> Self {
        Self {
            state: RwSignal::new(AuthState::default()),
        }
    }

    /// Create a store with pre-existing state (useful for SSR hydration).
    pub fn with_state(state: AuthState) -> Self {
        Self {
            state: RwSignal::new(state),
        }
    }

    // ========================================================================
    // Getters
    // ========================================================================

    /// Check if user is authenticated.
    pub fn is_authenticated(&self) -> bool {
        self.state.with(|s| s.is_authenticated())
    }

    /// Get current user if logged in.
    pub fn current_user(&self) -> Option<User> {
        self.state.with(|s| s.user.clone())
    }

    /// Get user's display name or a default.
    pub fn display_name(&self) -> String {
        self.state.with(|s| {
            s.user
                .as_ref()
                .map(|u| u.name.clone())
                .unwrap_or_else(|| "Guest".to_string())
        })
    }

    /// Get user's email if logged in.
    pub fn user_email(&self) -> Option<String> {
        self.state.with(|s| s.user.as_ref().map(|u| u.email.clone()))
    }

    /// Get user's initials for avatar placeholder.
    pub fn user_initials(&self) -> String {
        self.state.with(|s| {
            s.user
                .as_ref()
                .map(|u| {
                    u.name
                        .split_whitespace()
                        .filter_map(|word| word.chars().next())
                        .take(2)
                        .collect::<String>()
                        .to_uppercase()
                })
                .unwrap_or_else(|| "?".to_string())
        })
    }

    /// Check if loading.
    pub fn is_loading(&self) -> bool {
        self.state.with(|s| s.loading)
    }

    /// Get the current error.
    pub fn error(&self) -> Option<AuthError> {
        self.state.with(|s| s.error.clone())
    }

    /// Check if there's an error.
    pub fn has_error(&self) -> bool {
        self.state.with(|s| s.error.is_some())
    }

    // ========================================================================
    // Mutators
    // ========================================================================

    /// Set the current user.
    pub fn set_user(&self, user: Option<User>) {
        self.state.update(|s| s.user = user);
    }

    /// Set the authentication token.
    pub fn set_token(&self, token: Option<AuthToken>) {
        self.state.update(|s| s.token = token);
    }

    /// Set loading state.
    pub fn set_loading(&self, loading: bool) {
        self.state.update(|s| s.loading = loading);
    }

    /// Set error state.
    pub fn set_error(&self, error: Option<AuthError>) {
        self.state.update(|s| s.error = error);
    }

    /// Clear error state.
    pub fn clear_error(&self) {
        self.state.update(|s| s.error = None);
    }

    /// Set remember me preference.
    pub fn set_remember_me(&self, remember: bool) {
        self.state.update(|s| s.remember_me = remember);
    }

    /// Set authenticated state (user + token together).
    pub fn set_authenticated(&self, user: User, token: AuthToken) {
        self.state.update(|s| {
            s.user = Some(user);
            s.token = Some(token);
            s.error = None;
            s.loading = false;
        });
    }

    /// Clear all authentication state (logout).
    pub fn clear_auth(&self) {
        self.state.update(|s| {
            s.user = None;
            s.token = None;
            s.error = None;
            s.loading = false;
        });
    }

    // ========================================================================
    // Actions
    // ========================================================================

    /// Perform login action.
    ///
    /// This is a simulated login - in a real app, this would call an API.
    pub fn login(&self, credentials: LoginCredentials) {
        self.set_loading(true);
        self.clear_error();

        // Simulate API call delay would happen in async action
        // For demo, we do synchronous validation

        // Validate credentials
        if credentials.email.is_empty() {
            self.set_error(Some(AuthError::Validation(
                "Email is required".to_string(),
            )));
            self.set_loading(false);
            return;
        }

        if credentials.password.is_empty() {
            self.set_error(Some(AuthError::Validation(
                "Password is required".to_string(),
            )));
            self.set_loading(false);
            return;
        }

        // Simulate successful login
        // In a real app, this would be an async API call
        let user = User {
            id: "user_123".to_string(),
            email: credentials.email.clone(),
            name: credentials
                .email
                .split('@')
                .next()
                .unwrap_or("User")
                .to_string(),
            avatar_url: None,
        };

        let token = AuthToken {
            access_token: "mock_access_token_xyz".to_string(),
            refresh_token: Some("mock_refresh_token_abc".to_string()),
            expires_at: 3600, // 1 hour
        };

        self.set_remember_me(credentials.remember_me);
        self.set_authenticated(user, token);
    }

    /// Perform logout action.
    pub fn logout(&self) {
        self.set_loading(true);

        // In a real app, you might want to:
        // 1. Call logout API
        // 2. Clear local storage
        // 3. Clear cookies

        self.clear_auth();
    }

    /// Attempt to restore session from storage.
    ///
    /// In a real app, this would check localStorage/cookies.
    pub fn restore_session(&self) -> bool {
        // Simulated - in real app, check storage
        false
    }
}

impl Store for AuthStore {
    type State = AuthState;

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
    fn test_auth_store_creation() {
        let store = AuthStore::new();
        assert!(!store.is_authenticated());
        assert!(!store.is_loading());
        assert!(!store.has_error());
    }

    #[test]
    fn test_auth_store_getters() {
        let store = AuthStore::new();

        // Default state
        assert_eq!(store.display_name(), "Guest");
        assert_eq!(store.user_initials(), "?");
        assert!(store.current_user().is_none());
        assert!(store.user_email().is_none());
    }

    #[test]
    fn test_auth_store_set_user() {
        let store = AuthStore::new();

        let user = User {
            id: "1".to_string(),
            email: "test@example.com".to_string(),
            name: "John Doe".to_string(),
            avatar_url: None,
        };

        store.set_user(Some(user));

        assert_eq!(store.display_name(), "John Doe");
        assert_eq!(store.user_initials(), "JD");
        assert_eq!(store.user_email(), Some("test@example.com".to_string()));
    }

    #[test]
    fn test_auth_store_set_authenticated() {
        let store = AuthStore::new();

        let user = User {
            id: "1".to_string(),
            email: "test@example.com".to_string(),
            name: "Jane Smith".to_string(),
            avatar_url: None,
        };

        let token = AuthToken {
            access_token: "token123".to_string(),
            refresh_token: None,
            expires_at: 3600,
        };

        store.set_authenticated(user, token);

        assert!(store.is_authenticated());
        assert!(!store.is_loading());
        assert!(!store.has_error());
    }

    #[test]
    fn test_auth_store_clear_auth() {
        let store = AuthStore::new();

        // Set up authenticated state
        let user = User {
            id: "1".to_string(),
            email: "test@example.com".to_string(),
            name: "Test User".to_string(),
            avatar_url: None,
        };

        let token = AuthToken {
            access_token: "token".to_string(),
            refresh_token: None,
            expires_at: 3600,
        };

        store.set_authenticated(user, token);
        assert!(store.is_authenticated());

        // Clear auth
        store.clear_auth();
        assert!(!store.is_authenticated());
        assert!(store.current_user().is_none());
    }

    #[test]
    fn test_auth_store_login_validation() {
        let store = AuthStore::new();

        // Empty email
        store.login(LoginCredentials {
            email: "".to_string(),
            password: "password".to_string(),
            remember_me: false,
        });

        assert!(store.has_error());
        assert!(!store.is_authenticated());

        store.clear_error();

        // Empty password
        store.login(LoginCredentials {
            email: "test@example.com".to_string(),
            password: "".to_string(),
            remember_me: false,
        });

        assert!(store.has_error());
        assert!(!store.is_authenticated());
    }

    #[test]
    fn test_auth_store_login_success() {
        let store = AuthStore::new();

        store.login(LoginCredentials {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            remember_me: true,
        });

        assert!(store.is_authenticated());
        assert!(!store.has_error());
        assert_eq!(store.user_email(), Some("test@example.com".to_string()));
    }

    #[test]
    fn test_auth_store_logout() {
        let store = AuthStore::new();

        // Login first
        store.login(LoginCredentials {
            email: "test@example.com".to_string(),
            password: "password".to_string(),
            remember_me: false,
        });

        assert!(store.is_authenticated());

        // Logout
        store.logout();

        assert!(!store.is_authenticated());
    }

    #[test]
    fn test_user_initials() {
        let store = AuthStore::new();

        // Single word name
        store.set_user(Some(User {
            id: "1".to_string(),
            email: "test@example.com".to_string(),
            name: "Alice".to_string(),
            avatar_url: None,
        }));
        assert_eq!(store.user_initials(), "A");

        // Two word name
        store.set_user(Some(User {
            id: "1".to_string(),
            email: "test@example.com".to_string(),
            name: "Bob Smith".to_string(),
            avatar_url: None,
        }));
        assert_eq!(store.user_initials(), "BS");

        // Three word name (takes first two)
        store.set_user(Some(User {
            id: "1".to_string(),
            email: "test@example.com".to_string(),
            name: "Charlie David Evans".to_string(),
            avatar_url: None,
        }));
        assert_eq!(store.user_initials(), "CD");
    }

    #[test]
    fn test_auth_error_display() {
        assert_eq!(AuthError::InvalidCredentials.to_string(), "Invalid credentials");
        assert_eq!(AuthError::UserNotFound.to_string(), "User not found");
        assert_eq!(AuthError::EmailExists.to_string(), "Email already exists");
        assert_eq!(AuthError::TokenExpired.to_string(), "Token expired");
        assert_eq!(
            AuthError::Network("Connection failed".to_string()).to_string(),
            "Network error: Connection failed"
        );
    }
}
