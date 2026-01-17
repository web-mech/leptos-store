# ============================================================================
# leptos-store Makefile
# ============================================================================
# Enterprise-grade, type-enforced state management for Leptos
# ============================================================================

.PHONY: all build check test test-all clippy fmt fmt-check doc doc-open \
        clean example example-build example-release publish publish-dry \
        deps audit outdated help

# Colors for terminal output
CYAN := \033[36m
GREEN := \033[32m
YELLOW := \033[33m
RED := \033[31m
RESET := \033[0m

# Default target
all: check test

# ============================================================================
# Building
# ============================================================================

## Build the library in debug mode
build:
	@echo "$(CYAN)Building leptos-store...$(RESET)"
	cargo build

## Build the library in release mode
build-release:
	@echo "$(CYAN)Building leptos-store (release)...$(RESET)"
	cargo build --release

## Check the library compiles without building
check:
	@echo "$(CYAN)Checking leptos-store...$(RESET)"
	cargo check --workspace

## Check all feature combinations
check-features:
	@echo "$(CYAN)Checking all feature combinations...$(RESET)"
	cargo check --no-default-features
	cargo check --features ssr
	cargo check --features hydrate
	cargo check --features csr
	cargo check --all-features

# ============================================================================
# Testing
# ============================================================================

## Run all tests
test:
	@echo "$(CYAN)Running tests...$(RESET)"
	cargo test --workspace

## Run tests with output
test-verbose:
	@echo "$(CYAN)Running tests (verbose)...$(RESET)"
	cargo test --workspace -- --nocapture

## Run only library tests
test-lib:
	@echo "$(CYAN)Running library tests...$(RESET)"
	cargo test --lib

## Run only doc tests
test-doc:
	@echo "$(CYAN)Running doc tests...$(RESET)"
	cargo test --doc

## Run tests for the example
test-example:
	@echo "$(CYAN)Running example tests...$(RESET)"
	cargo test -p auth-store-example

## Run all tests with all features
test-all:
	@echo "$(CYAN)Running all tests with all features...$(RESET)"
	cargo test --workspace --all-features

## Run tests with coverage (requires cargo-tarpaulin)
coverage:
	@echo "$(CYAN)Running tests with coverage...$(RESET)"
	cargo tarpaulin --workspace --out Html --output-dir target/coverage

# ============================================================================
# Code Quality
# ============================================================================

## Run clippy lints
clippy:
	@echo "$(CYAN)Running clippy...$(RESET)"
	cargo clippy --workspace --all-targets -- -D warnings

## Run clippy with fixes
clippy-fix:
	@echo "$(CYAN)Running clippy with fixes...$(RESET)"
	cargo clippy --workspace --all-targets --fix --allow-dirty

## Format code
fmt:
	@echo "$(CYAN)Formatting code...$(RESET)"
	cargo fmt --all

## Check code formatting
fmt-check:
	@echo "$(CYAN)Checking code format...$(RESET)"
	cargo fmt --all -- --check

## Run all quality checks (CI pipeline)
ci: fmt-check clippy test-all
	@echo "$(GREEN)All CI checks passed!$(RESET)"

# ============================================================================
# Documentation
# ============================================================================

## Build documentation
doc:
	@echo "$(CYAN)Building documentation...$(RESET)"
	cargo doc --workspace --no-deps

## Build and open documentation
doc-open:
	@echo "$(CYAN)Building and opening documentation...$(RESET)"
	cargo doc --workspace --no-deps --open

## Build documentation with private items
doc-private:
	@echo "$(CYAN)Building documentation (including private items)...$(RESET)"
	cargo doc --workspace --no-deps --document-private-items

# ============================================================================
# Example Application
# ============================================================================

## Install cargo-leptos if not present
leptos-install:
	@echo "$(CYAN)Installing cargo-leptos...$(RESET)"
	@command -v cargo-leptos >/dev/null 2>&1 || cargo install cargo-leptos

## Install trunk (WASM bundler) if not present
trunk-install:
	@echo "$(CYAN)Installing trunk...$(RESET)"
	@command -v trunk >/dev/null 2>&1 || cargo install trunk

## Add WASM target if not present
wasm-target:
	@echo "$(CYAN)Adding wasm32-unknown-unknown target...$(RESET)"
	rustup target add wasm32-unknown-unknown

## Setup example dependencies
example-setup: wasm-target
	@echo "$(GREEN)Example setup complete!$(RESET)"

## Run the example in SSR mode (recommended)
example: example-setup leptos-install
	@echo "$(CYAN)Starting auth-store-example (SSR mode)...$(RESET)"
	@echo "$(YELLOW)Open http://127.0.0.1:3000 in your browser$(RESET)"
	cd examples/auth-store-example && cargo leptos watch

## Run the example in SSR mode (manual - without cargo-leptos)
example-ssr: example-setup
	@echo "$(CYAN)Building WASM for hydration...$(RESET)"
	cd examples/auth-store-example && cargo build --lib --features hydrate --target wasm32-unknown-unknown
	@echo "$(CYAN)Starting server...$(RESET)"
	@echo "$(YELLOW)Open http://127.0.0.1:3000 in your browser$(RESET)"
	cd examples/auth-store-example && cargo run --features ssr

## Run the example in CSR mode (with trunk)
example-csr: example-setup trunk-install
	@echo "$(CYAN)Starting auth-store-example (CSR mode)...$(RESET)"
	@echo "$(YELLOW)Open http://localhost:8080 in your browser$(RESET)"
	cd examples/auth-store-example && trunk serve --features csr

## Run the example on a specific port (CSR mode)
example-port: example-setup trunk-install
	@echo "$(CYAN)Starting auth-store-example on port $(PORT)...$(RESET)"
	cd examples/auth-store-example && trunk serve --features csr --port $(PORT)

## Build the example for production (SSR)
example-build: example-setup leptos-install
	@echo "$(CYAN)Building auth-store-example (SSR)...$(RESET)"
	cd examples/auth-store-example && cargo leptos build

## Build the example for production (SSR, optimized)
example-release: example-setup leptos-install
	@echo "$(CYAN)Building auth-store-example (SSR, release)...$(RESET)"
	cd examples/auth-store-example && cargo leptos build --release

## Build the example for CSR
example-build-csr: example-setup trunk-install
	@echo "$(CYAN)Building auth-store-example (CSR)...$(RESET)"
	cd examples/auth-store-example && trunk build --features csr

## Build the example for CSR (optimized)
example-release-csr: example-setup trunk-install
	@echo "$(CYAN)Building auth-store-example (CSR, release)...$(RESET)"
	cd examples/auth-store-example && trunk build --release --features csr

## Clean example build artifacts
example-clean:
	@echo "$(CYAN)Cleaning example build artifacts...$(RESET)"
	rm -rf examples/auth-store-example/dist
	rm -rf examples/auth-store-example/target/site

# ============================================================================
# Publishing
# ============================================================================

## Dry run publish to crates.io
publish-dry:
	@echo "$(CYAN)Dry run publish to crates.io...$(RESET)"
	cargo publish --dry-run

## Publish to crates.io
publish:
	@echo "$(YELLOW)Publishing to crates.io...$(RESET)"
	@echo "$(RED)Make sure you have:$(RESET)"
	@echo "  1. Updated version in Cargo.toml"
	@echo "  2. Updated CHANGELOG.md"
	@echo "  3. Committed all changes"
	@echo "  4. Created a git tag"
	@read -p "Continue? [y/N] " confirm && [ "$$confirm" = "y" ] && cargo publish

## Create a new version tag
tag:
	@echo "$(CYAN)Creating version tag...$(RESET)"
	@VERSION=$$(grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)"/\1/'); \
	echo "Creating tag v$$VERSION"; \
	git tag -a "v$$VERSION" -m "Release v$$VERSION"

## Push tags to remote
tag-push:
	@echo "$(CYAN)Pushing tags to remote...$(RESET)"
	git push --tags

# ============================================================================
# Dependencies
# ============================================================================

## Update dependencies
deps-update:
	@echo "$(CYAN)Updating dependencies...$(RESET)"
	cargo update

## Check for outdated dependencies (requires cargo-outdated)
outdated:
	@echo "$(CYAN)Checking for outdated dependencies...$(RESET)"
	cargo outdated --workspace

## Audit dependencies for security vulnerabilities (requires cargo-audit)
audit:
	@echo "$(CYAN)Auditing dependencies...$(RESET)"
	cargo audit

## Show dependency tree
deps-tree:
	@echo "$(CYAN)Showing dependency tree...$(RESET)"
	cargo tree

## Show dependency tree for a specific package
deps-tree-pkg:
	@echo "$(CYAN)Showing dependency tree for $(PKG)...$(RESET)"
	cargo tree -p $(PKG)

# ============================================================================
# Utilities
# ============================================================================

## Clean all build artifacts
clean:
	@echo "$(CYAN)Cleaning build artifacts...$(RESET)"
	cargo clean
	rm -rf examples/auth-store-example/dist

## Watch for changes and run tests
watch:
	@echo "$(CYAN)Watching for changes...$(RESET)"
	cargo watch -x test

## Watch for changes and run clippy
watch-clippy:
	@echo "$(CYAN)Watching for changes (clippy)...$(RESET)"
	cargo watch -x clippy

## Generate a new store template
new-store:
	@echo "$(CYAN)Store template:$(RESET)"
	@echo ""
	@echo "use leptos::prelude::*;"
	@echo "use leptos_store::prelude::*;"
	@echo ""
	@echo "#[derive(Clone, Debug, Default)]"
	@echo "pub struct MyState {"
	@echo "    // Add your state fields here"
	@echo "}"
	@echo ""
	@echo "#[derive(Clone)]"
	@echo "pub struct MyStore {"
	@echo "    state: RwSignal<MyState>,"
	@echo "}"
	@echo ""
	@echo "impl MyStore {"
	@echo "    pub fn new() -> Self {"
	@echo "        Self {"
	@echo "            state: RwSignal::new(MyState::default()),"
	@echo "        }"
	@echo "    }"
	@echo ""
	@echo "    // Add getters here"
	@echo ""
	@echo "    // Add mutators here"
	@echo "}"
	@echo ""
	@echo "impl Store for MyStore {"
	@echo "    type State = MyState;"
	@echo ""
	@echo "    fn state(&self) -> ReadSignal<Self::State> {"
	@echo "        self.state.read_only()"
	@echo "    }"
	@echo "}"

## Show lines of code statistics
loc:
	@echo "$(CYAN)Lines of code:$(RESET)"
	@find src -name "*.rs" | xargs wc -l | tail -1
	@echo ""
	@echo "$(CYAN)By file:$(RESET)"
	@find src -name "*.rs" | xargs wc -l | sort -n

# ============================================================================
# Help
# ============================================================================

## Show this help message
help:
	@echo "$(CYAN)leptos-store Makefile$(RESET)"
	@echo ""
	@echo "$(GREEN)Usage:$(RESET)"
	@echo "  make [target]"
	@echo ""
	@echo "$(GREEN)Building:$(RESET)"
	@echo "  build          Build the library in debug mode"
	@echo "  build-release  Build the library in release mode"
	@echo "  check          Check the library compiles"
	@echo "  check-features Check all feature combinations"
	@echo ""
	@echo "$(GREEN)Testing:$(RESET)"
	@echo "  test           Run all tests"
	@echo "  test-verbose   Run tests with output"
	@echo "  test-lib       Run only library tests"
	@echo "  test-doc       Run only doc tests"
	@echo "  test-example   Run example tests"
	@echo "  test-all       Run all tests with all features"
	@echo "  coverage       Run tests with coverage"
	@echo ""
	@echo "$(GREEN)Code Quality:$(RESET)"
	@echo "  clippy         Run clippy lints"
	@echo "  clippy-fix     Run clippy with auto-fixes"
	@echo "  fmt            Format code"
	@echo "  fmt-check      Check code formatting"
	@echo "  ci             Run all CI checks"
	@echo ""
	@echo "$(GREEN)Documentation:$(RESET)"
	@echo "  doc            Build documentation"
	@echo "  doc-open       Build and open documentation"
	@echo "  doc-private    Build docs with private items"
	@echo ""
	@echo "$(GREEN)Example (SSR mode - recommended):$(RESET)"
	@echo "  example          Run the example (SSR with cargo-leptos)"
	@echo "  example-ssr      Run the example (SSR manual)"
	@echo "  example-build    Build the example (SSR)"
	@echo "  example-release  Build the example (SSR, optimized)"
	@echo ""
	@echo "$(GREEN)Example (CSR mode):$(RESET)"
	@echo "  example-csr        Run the example (CSR with trunk)"
	@echo "  example-build-csr  Build the example (CSR)"
	@echo "  example-release-csr Build the example (CSR, optimized)"
	@echo "  example-clean      Clean example artifacts"
	@echo ""
	@echo "$(GREEN)Publishing:$(RESET)"
	@echo "  publish-dry    Dry run publish"
	@echo "  publish        Publish to crates.io"
	@echo "  tag            Create version tag"
	@echo "  tag-push       Push tags to remote"
	@echo ""
	@echo "$(GREEN)Dependencies:$(RESET)"
	@echo "  deps-update    Update dependencies"
	@echo "  outdated       Check for outdated deps"
	@echo "  audit          Audit for vulnerabilities"
	@echo "  deps-tree      Show dependency tree"
	@echo ""
	@echo "$(GREEN)Utilities:$(RESET)"
	@echo "  clean          Clean all build artifacts"
	@echo "  watch          Watch and run tests"
	@echo "  watch-clippy   Watch and run clippy"
	@echo "  new-store      Print store template"
	@echo "  loc            Show lines of code"
	@echo "  help           Show this help"
