//! Server entry point for the Auth Store Example
//!
//! This runs the SSR server with Actix Web.

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use auth_store_example::components::App;
    use leptos::prelude::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};

    // Get Leptos configuration from Cargo.toml
    let conf = get_configuration(Some("Cargo.toml")).unwrap();
    let addr = conf.leptos_options.site_addr;

    println!("🚀 Auth Store Example (SSR Mode)");
    println!("   Listening on http://{}", addr);

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = leptos_options.site_root.clone();

        // Generate the list of routes
        let routes = generate_route_list(App);

        App::new()
            // Serve static files from the `pkg` directory
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            // Leptos routes
            .leptos_routes(routes, {
                let leptos_options = leptos_options.clone();
                move || {
                    view! {
                        <!DOCTYPE html>
                        <html lang="en">
                            <head>
                                <meta charset="utf-8"/>
                                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                                <AutoReload options=leptos_options.clone() />
                                <HydrationScripts options=leptos_options.clone() />
                                <leptos_meta::MetaTags/>
                            </head>
                            <body>
                                <App/>
                            </body>
                        </html>
                    }
                }
            })
            .app_data(web::Data::new(leptos_options.clone()))
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(not(feature = "ssr"))]
fn main() {
    // This binary requires the `ssr` feature
    eprintln!("This binary requires the `ssr` feature. Run with:");
    eprintln!("  cargo run --features ssr");
}
