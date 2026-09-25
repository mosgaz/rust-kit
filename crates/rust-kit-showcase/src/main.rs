#[cfg(feature = "ssr")]
use leptos::prelude::*;

#[cfg(feature = "ssr")]
use leptos_axum::{generate_route_list, LeptosRoutes};

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use showcase::App;

    let conf = get_configuration(None).expect("configuration");
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("bind");
    axum::serve(listener, app.into_make_service())
        .await
        .expect("serve");
}

#[cfg(feature = "ssr")]
fn shell(options: LeptosOptions) -> impl IntoView {
    use leptos_meta::{provide_meta_context, MetaTags, Stylesheet};
    use showcase::App;

    provide_meta_context();

    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1.0, minimum-scale=1.0, maximum-scale=1.0, user-scalable=no, viewport-fit=cover"/>
                <MetaTags/>
                <Stylesheet id="leptos" href="/pkg/showcase.css"/>

                // Google Fonts
                <link rel="preconnect" href="https://fonts.googleapis.com"/>
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous"/>
                <link
                    rel="stylesheet"
                    href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Geist:wght@400;500;600;700&family=Roboto:wght@400;500;600;700&family=DM+Sans:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500;600;700&display=swap"
                    media="print"
                    onload="this.media='all'"
                />
                <noscript>
                    <link
                        rel="stylesheet"
                        href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Geist:wght@400;500;600;700&family=Roboto:wght@400;500;600;700&family=DM+Sans:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500;600;700&display=swap"
                    />
                </noscript>

                // Dark mode (must run before render to avoid flash)
                <script>
                    "if(localStorage.getItem('darkmode')==='true'||(localStorage.getItem('darkmode')===null&&window.matchMedia('(prefers-color-scheme:dark)').matches)){document.documentElement.classList.add('dark')}"
                </script>

                // Sonner CSS (non-critical)
                <link rel="preload" href="/app_components/sonner.css" r#as="style"/>
                <link rel="stylesheet" href="/app_components/sonner.css" media="print" onload="this.media='all'"/>
                <noscript>
                    <link rel="stylesheet" href="/app_components/sonner.css"/>
                </noscript>

                // Async scripts
                <script async src="/app_components/resizable.bundle.js"></script>
                <script async src="/app_components/shimmer_init.js?v=3"></script>
                <script async src="/app_components/lazy_load_sonner.js"></script>

                // JSON-LD structured data for SEO
                <script type="application/ld+json" inner_html=include_str!("../public/schema.json")></script>

                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[cfg(not(feature = "ssr"))]
fn main() {
    // WASM: точка входа через wasm_bindgen::hydrate в lib.rs
}
