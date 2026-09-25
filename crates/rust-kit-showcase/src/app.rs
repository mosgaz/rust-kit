use app_routes::{ComponentsRoutes, HooksRoutes};
use leptos::prelude::*;
use leptos_meta::{Html, provide_meta_context};
use leptos_router::components::{ParentRoute, Route, Router, Routes};
use leptos_router::{ParamSegment, StaticSegment};
use registry::hooks::use_theme_mode::ThemeMode;
use registry::ui::sonner::SonnerToaster;
use registry::ui::toast_custom::toaster::{Toaster, provide_toaster};

use crate::components::navigation::app_wrapper::AppWrapper;
use crate::domain::docs::routing::docs_layout::DocsLayout;
use crate::domain::docs::routing::page_all_demos::PageAllDemos;
use crate::domain::docs::routing::shared_routes_demo::SharedRoutesDemo;
use crate::routes::page_not_found::PageNotFound;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_toaster();

    // RequestUrl must be provided INSIDE App (not in hydrate()),
    // because Router reads it from the current Owner context,
    // and hydrate_body creates that Owner only when rendering App.

    #[cfg(feature = "hydrate")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(href) = window.location().href() {
                provide_context(leptos_router::location::RequestUrl::new(&href));
            }
        }
    }

    let theme_mode = ThemeMode::init();

    view! {
        <Html {..} class=move || if theme_mode.is_dark() { "dark" } else { "" } />
        <Router>
            <Toaster />
            <SonnerToaster />
            <AppWrapper>
                <main class="overflow-y-auto flex-1 overflow-x-clip">
                    <Routes fallback=|| PageNotFound.into_view()>
                        <ParentRoute path=StaticSegment("docs") view=DocsLayout>
                            <Route
                                path=StaticSegment(ComponentsRoutes::segment())
                                view=|| view! { <PageAllDemos segment="components" /> }
                            />
                            <Route
                                path=(StaticSegment(ComponentsRoutes::segment()), ParamSegment("name"))
                                view=|| view! { <SharedRoutesDemo route_path=ComponentsRoutes::base_url() /> }
                            />
                            <Route
                                path=StaticSegment(HooksRoutes::segment())
                                view=|| view! { <PageAllDemos segment="hooks" /> }
                            />
                            <Route
                                path=(StaticSegment(HooksRoutes::segment()), ParamSegment("name"))
                                view=|| view! { <SharedRoutesDemo route_path=HooksRoutes::base_url() /> }
                            />
                        </ParentRoute>
                    </Routes>
                </main>
            </AppWrapper>
        </Router>
    }
}
