//! UI Components for the Token Explorer Example
//!
//! Displays Solana token data with price changes, stats, and filtering.

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use leptos_store::prelude::*;

use crate::token_store::{SortField, Token, TokenStore};

/// Read hydration data from a script tag in the DOM
#[cfg(feature = "hydrate")]
fn read_hydration_script(store_key: &str) -> Option<String> {
    use wasm_bindgen::JsCast;
    
    let window = web_sys::window()?;
    let document = window.document()?;
    let script_id = format!("__leptos_store_{}", store_key);
    let element = document.get_element_by_id(&script_id)?;
    let script = element.dyn_into::<web_sys::HtmlScriptElement>().ok()?;
    let text = script.text().ok()?;
    Some(text)
}

/// Format number with thousands separator
fn format_with_commas(n: u64) -> String {
    let s = n.to_string();
    let bytes: Vec<_> = s.bytes().rev().collect();
    let chunks: Vec<_> = bytes
        .chunks(3)
        .map(|chunk| String::from_utf8(chunk.to_vec()).unwrap())
        .collect();
    chunks.join(",").chars().rev().collect()
}

/// Main application shell
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // On server: Store is already provided by main.rs with pre-fetched tokens
    // On client (hydrate): Read serialized state and create store
    #[cfg(feature = "hydrate")]
    {
        use leptos_store::hydration::HydratableStore;
        
        // Try to hydrate from serialized data, fallback to empty store
        let store = if let Some(data) = read_hydration_script("token_store") {
            TokenStore::from_hydrated_state(&data).unwrap_or_else(|_| TokenStore::new())
        } else {
            TokenStore::new()
        };
        provide_store(store);
    }

    // On CSR (no SSR): just create empty store
    #[cfg(all(not(feature = "hydrate"), not(feature = "ssr")))]
    {
        provide_store(TokenStore::new());
    }

    view! {
        <Stylesheet id="leptos" href="/pkg/token-explorer-example.css"/>
        <Title text="Token Explorer - Solana Tokens"/>
        <Meta name="description" content="Explore Solana tokens with real-time data"/>

        <Router>
            <main class="app">
                <Routes fallback=|| "Page not found">
                    <Route path=path!("/") view=TokenExplorer/>
                </Routes>
            </main>
        </Router>
    }
}

/// Main token explorer page
#[component]
fn TokenExplorer() -> impl IntoView {
    view! {
        <div class="token-explorer">
            <Header />
            <SearchAndFilter />
            <TokenGrid />
            <TokenDetail />
        </div>
    }
}

/// Page header
#[component]
fn Header() -> impl IntoView {
    let store = use_store::<TokenStore>();

    view! {
        <header class="header">
            <div class="header-content">
                <div class="logo">
                    <span class="logo-icon">"◎"</span>
                    <h1>"Token Explorer"</h1>
                </div>
                <div class="header-stats">
                    <span class="stat">
                        <span class="stat-label">"Tokens"</span>
                        <span class="stat-value">{move || store.token_count()}</span>
                    </span>
                </div>
            </div>
        </header>
    }
}

/// Search and filter controls
#[component]
fn SearchAndFilter() -> impl IntoView {
    let store = use_store::<TokenStore>();
    let store_search = store.clone();

    view! {
        <div class="controls">
            <div class="search-box">
                <input
                    type="text"
                    placeholder="Search tokens..."
                    prop:value=move || store.search_query()
                    on:input:target=move |ev| {
                        store_search.set_search_query(ev.target().value());
                    }
                />
            </div>
            <div class="sort-buttons">
                <SortButton field=SortField::MarketCap label="MCap" />
                <SortButton field=SortField::PriceChange24h label="24h %" />
                <SortButton field=SortField::Liquidity label="Liq" />
                <SortButton field=SortField::Holders label="Holders" />
            </div>
        </div>
    }
}

/// Sort button component
#[component]
fn SortButton(field: SortField, label: &'static str) -> impl IntoView {
    let store = use_store::<TokenStore>();
    let field_clone = field.clone();
    let field_for_click = field.clone();
    let store_active = store.clone();
    let store_desc = store.clone();
    let store_click = store.clone();

    view! {
        <button
            class="sort-btn"
            class:active=move || store_active.sort_by() == field_clone.clone()
            on:click=move |_| store_click.set_sort_by(field_for_click.clone())
        >
            {label}
            {move || {
                if store.sort_by() == field.clone() {
                    if store_desc.is_sort_desc() { " ↓" } else { " ↑" }
                } else {
                    ""
                }
            }}
        </button>
    }
}

/// Token grid display
#[component]
fn TokenGrid() -> impl IntoView {
    let store = use_store::<TokenStore>();

    view! {
        <div class="token-grid">
            {move || {
                let tokens = store.filtered_tokens();
                if tokens.is_empty() {
                    view! {
                        <div class="empty-state">
                            <p>"No tokens found"</p>
                        </div>
                    }.into_any()
                } else {
                    tokens.into_iter().map(|token| {
                        view! { <TokenCard token=token /> }
                    }).collect_view().into_any()
                }
            }}
        </div>
    }
}

/// Individual token card
#[component]
fn TokenCard(token: Token) -> impl IntoView {
    let store = use_store::<TokenStore>();
    let token_id = token.id.clone();

    let price_change_24h = token.price_change_24h();
    let price_change_1h = token.price_change_1h();
    let is_positive_24h = price_change_24h >= 0.0;
    let is_positive_1h = price_change_1h >= 0.0;

    let icon_url = token.icon.clone().unwrap_or_default();
    let has_icon = !icon_url.is_empty();
    
    let token_name = token.name.clone();
    let token_symbol = token.symbol.clone();
    let token_symbol_icon = token.symbol.clone();
    let formatted_price = token.formatted_price();
    let formatted_mcap = token.formatted_mcap();
    let formatted_liquidity = token.formatted_liquidity();
    let holder_count = format!("{}", token.holder_count);
    let short_address = token.short_address();
    let full_id = token.id.clone();
    let launchpad = token.launchpad.clone();
    let is_verified = token.is_verified();

    view! {
        <div
            class="token-card"
            on:click=move |_| store.select_token(Some(token_id.clone()))
        >
            <div class="token-header">
                <div class="token-icon">
                    {if has_icon {
                        view! { <img src=icon_url.clone() alt=token_symbol_icon.clone() /> }.into_any()
                    } else {
                        let first_char = token_symbol_icon.chars().next().unwrap_or('?').to_string();
                        view! { <span class="token-icon-placeholder">{first_char}</span> }.into_any()
                    }}
                </div>
                <div class="token-info">
                    <h3 class="token-name">{token_name}</h3>
                    <span class="token-symbol">{token_symbol}</span>
                </div>
                {if is_verified {
                    view! { <span class="verified-badge" title="Verified">"✓"</span> }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
            </div>

            <div class="token-price">
                <span class="price">{formatted_price}</span>
                <span
                    class="price-change"
                    class:positive=is_positive_24h
                    class:negative=!is_positive_24h
                >
                    {if is_positive_24h { "+" } else { "" }}
                    {format!("{:.2}%", price_change_24h)}
                </span>
            </div>

            <div class="token-stats">
                <div class="stat-row">
                    <span class="stat-label">"MCap"</span>
                    <span class="stat-value">{formatted_mcap}</span>
                </div>
                <div class="stat-row">
                    <span class="stat-label">"Liquidity"</span>
                    <span class="stat-value">{formatted_liquidity}</span>
                </div>
                <div class="stat-row">
                    <span class="stat-label">"Holders"</span>
                    <span class="stat-value">{holder_count}</span>
                </div>
                <div class="stat-row">
                    <span class="stat-label">"1h"</span>
                    <span
                        class="stat-value"
                        class:positive=is_positive_1h
                        class:negative=!is_positive_1h
                    >
                        {if is_positive_1h { "+" } else { "" }}
                        {format!("{:.2}%", price_change_1h)}
                    </span>
                </div>
            </div>

            <div class="token-footer">
                <span class="token-address" title=full_id>{short_address}</span>
                {launchpad.map(|lp| {
                    view! { <span class="launchpad-badge">{lp}</span> }
                })}
            </div>
        </div>
    }
}

/// Token detail modal/panel
#[component]
fn TokenDetail() -> impl IntoView {
    let store = use_store::<TokenStore>();
    let store_close = store.clone();
    let store_close2 = store.clone();

    view! {
        {move || {
            store.selected_token().map(|token| {
                let price_change_24h = token.price_change_24h();
                let is_positive = price_change_24h >= 0.0;
                let icon_url = token.icon.clone().unwrap_or_default();
                let has_icon = !icon_url.is_empty();
                
                // Pre-compute all values to avoid reference issues
                let token_name = token.name.clone();
                let token_symbol = token.symbol.clone();
                let token_symbol_icon = token.symbol.clone();
                let formatted_price = token.formatted_price();
                let formatted_mcap = token.formatted_mcap();
                let formatted_liquidity = token.formatted_liquidity();
                let holder_count = format_with_commas(token.holder_count);
                let total_supply = format!("{:.0}", token.total_supply);
                let token_id = token.id.clone();
                let twitter = token.twitter.clone();
                let website = token.website.clone();
                let stats_24h = token.stats_24h.clone();
                let audit = token.audit.clone();
                
                let store_close_inner = store_close.clone();
                let store_close_btn = store_close2.clone();

                view! {
                    <div class="token-detail-overlay" on:click=move |_| store_close_inner.clear_selection()>
                        <div class="token-detail" on:click=|ev| ev.stop_propagation()>
                            <button class="close-btn" on:click=move |_| store_close_btn.clear_selection()>"×"</button>

                            <div class="detail-header">
                                <div class="detail-icon">
                                    {if has_icon {
                                        view! { <img src=icon_url.clone() alt=token_symbol_icon.clone() /> }.into_any()
                                    } else {
                                        let chars: String = token_symbol_icon.chars().take(2).collect();
                                        view! { <span class="icon-placeholder">{chars}</span> }.into_any()
                                    }}
                                </div>
                                <div class="detail-title">
                                    <h2>{token_name}</h2>
                                    <span class="symbol">{token_symbol}</span>
                                </div>
                            </div>

                            <div class="detail-price">
                                <span class="big-price">{formatted_price}</span>
                                <span
                                    class="big-change"
                                    class:positive=is_positive
                                    class:negative=!is_positive
                                >
                                    {if is_positive { "▲" } else { "▼" }}
                                    {format!(" {:.2}% (24h)", price_change_24h.abs())}
                                </span>
                            </div>

                            <div class="detail-grid">
                                <div class="detail-stat">
                                    <span class="label">"Market Cap"</span>
                                    <span class="value">{formatted_mcap}</span>
                                </div>
                                <div class="detail-stat">
                                    <span class="label">"Liquidity"</span>
                                    <span class="value">{formatted_liquidity}</span>
                                </div>
                                <div class="detail-stat">
                                    <span class="label">"Holders"</span>
                                    <span class="value">{holder_count}</span>
                                </div>
                                <div class="detail-stat">
                                    <span class="label">"Total Supply"</span>
                                    <span class="value">{total_supply}</span>
                                </div>
                            </div>

                            {stats_24h.as_ref().map(|stats| {
                                let buy_vol = format!("${:.0}", stats.buy_volume);
                                let sell_vol = format!("${:.0}", stats.sell_volume);
                                let num_buys = stats.num_buys;
                                let num_sells = stats.num_sells;
                                view! {
                                    <div class="detail-section">
                                        <h3>"24h Trading Activity"</h3>
                                        <div class="detail-grid">
                                            <div class="detail-stat">
                                                <span class="label">"Buy Volume"</span>
                                                <span class="value positive">{buy_vol}</span>
                                            </div>
                                            <div class="detail-stat">
                                                <span class="label">"Sell Volume"</span>
                                                <span class="value negative">{sell_vol}</span>
                                            </div>
                                            <div class="detail-stat">
                                                <span class="label">"Buys"</span>
                                                <span class="value">{num_buys}</span>
                                            </div>
                                            <div class="detail-stat">
                                                <span class="label">"Sells"</span>
                                                <span class="value">{num_sells}</span>
                                            </div>
                                        </div>
                                    </div>
                                }
                            })}

                            {audit.as_ref().map(|audit| {
                                let mint_disabled = audit.mint_authority_disabled;
                                let freeze_disabled = audit.freeze_authority_disabled;
                                let top_holders = format!("{:.2}%", audit.top_holders_percentage);
                                let dev_balance = format!("{:.4}%", audit.dev_balance_percentage);
                                view! {
                                    <div class="detail-section">
                                        <h3>"Security Audit"</h3>
                                        <div class="audit-badges">
                                            <span class="audit-badge" class:safe=mint_disabled>
                                                {if mint_disabled { "✓ Mint Disabled" } else { "⚠ Mint Active" }}
                                            </span>
                                            <span class="audit-badge" class:safe=freeze_disabled>
                                                {if freeze_disabled { "✓ Freeze Disabled" } else { "⚠ Freeze Active" }}
                                            </span>
                                        </div>
                                        <div class="detail-grid">
                                            <div class="detail-stat">
                                                <span class="label">"Top Holders %"</span>
                                                <span class="value">{top_holders}</span>
                                            </div>
                                            <div class="detail-stat">
                                                <span class="label">"Dev Balance %"</span>
                                                <span class="value">{dev_balance}</span>
                                            </div>
                                        </div>
                                    </div>
                                }
                            })}

                            <div class="detail-footer">
                                <code class="full-address">{token_id}</code>
                                <div class="detail-links">
                                    {twitter.map(|url| {
                                        view! {
                                            <a href=url target="_blank" class="link-btn">"Twitter"</a>
                                        }
                                    })}
                                    {website.map(|url| {
                                        view! {
                                            <a href=url target="_blank" class="link-btn">"Website"</a>
                                        }
                                    })}
                                </div>
                            </div>
                        </div>
                    </div>
                }
            })
        }}
    }
}
