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
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <MetaTags/>
				<Stylesheet id="leptos" href="/pkg/showcase.css"/>
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