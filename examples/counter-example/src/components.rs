// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 web-mech

//! UI Components for the Counter Example
//!
//! This module provides Leptos components that demonstrate
//! how to use a CounterStore with leptos-store.

use leptos::prelude::*;
use leptos_meta::{Meta, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use leptos_store::prelude::*;

use crate::counter_store::CounterStore;

/// Main app component
#[component]
pub fn App() -> impl IntoView {
    // Provides context for <Title> and <Meta> components
    provide_meta_context();

    // Create and provide the counter store
    let store = CounterStore::new();
    provide_store(store);

    view! {
        <Stylesheet id="leptos" href="/pkg/counter-example.css"/>
        <Title text="Counter Example - leptos-store"/>
        <Meta name="description" content="Simple counter example using leptos-store"/>

        <Router>
            <main class="app">
                <Routes fallback=|| "Page not found">
                    <Route path=path!("/") view=CounterPage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Counter page component
#[component]
fn CounterPage() -> impl IntoView {
    view! {
        <div class="counter-page">
            <div class="counter-card">
                <h1>"Counter Store Example"</h1>
                <p class="subtitle">"Simple state management with leptos-store"</p>

                <Counter />

                <div class="code-hint">
                    <p>"Store structure:"</p>
                    <pre><code>{r#"impl CounterStore {
    // Getters
    pub fn doubled(&self) -> i32
    pub fn is_positive(&self) -> bool

    // Mutators
    pub fn increment(&self)
    pub fn decrement(&self)
    pub fn reset(&self)
}"#}</code></pre>
                </div>
            </div>
        </div>
    }
}

/// Counter display and controls
#[component]
fn Counter() -> impl IntoView {
    let store = use_store::<CounterStore>();

    // Clone store for each closure
    let store_count = store.clone();
    let store_doubled = store.clone();
    let store_status = store.clone();
    let store_inc = store.clone();
    let store_dec = store.clone();
    let store_reset = store.clone();

    view! {
        <div class="counter">
            // Current count display
            <div class="count-display">
                <span class="count-value">
                    {move || store_count.state().get().count}
                </span>
            </div>

            // Button controls
            <div class="button-group">
                <button
                    class="btn btn-decrement"
                    on:click=move |_| store_dec.decrement()
                    aria-label="Decrement"
                >
                    "−"
                </button>

                <button
                    class="btn btn-reset"
                    on:click=move |_| store_reset.reset()
                    aria-label="Reset"
                >
                    "Reset"
                </button>

                <button
                    class="btn btn-increment"
                    on:click=move |_| store_inc.increment()
                    aria-label="Increment"
                >
                    "+"
                </button>
            </div>

            // Derived values (getters)
            <div class="info-panel">
                <div class="info-item">
                    <span class="info-label">"Doubled:"</span>
                    <span class="info-value">{move || store_doubled.doubled()}</span>
                </div>
                <div class="info-item">
                    <span class="info-label">"Status:"</span>
                    <span class="info-value">
                        {move || {
                            if store_status.is_positive() {
                                "Positive ↑"
                            } else if store_status.is_negative() {
                                "Negative ↓"
                            } else {
                                "Zero"
                            }
                        }}
                    </span>
                </div>
            </div>
        </div>
    }
}
