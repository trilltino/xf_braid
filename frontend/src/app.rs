//! Main application component with routing

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router_macro::path;

use crate::components::{Footer, Navbar};
use crate::pages::{
    BlogPage, CodePage, CommunityPage, DocsPage, FeaturesPage, HomePage, XFMailDemoPage, XFMailPage,
};

/// HTML shell for SSR - serves the initial HTML page
#[cfg(feature = "ssr")]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <meta name="description" content="XFBraid - Real-time Collaboration Platform"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <link rel="stylesheet" id="leptos" href="/pkg/xfbraid.css"/>
                <link rel="icon" type="image/x-icon" href="/favicon.ico"/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

/// Root application component
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="XFBraid"/>
        <Router>
            <div class="app">
                <Navbar />
                <main class="main-content">
                    <Routes fallback=|| view! { <p>"Page not found"</p> }>
                        <Route path=path!("/") view=HomePage />
                        <Route path=path!("/features") view=FeaturesPage />
                        <Route path=path!("/docs") view=DocsPage />
                        <Route path=path!("/community") view=CommunityPage />
                        <Route path=path!("/code") view=CodePage />
                        <Route path=path!("/blog") view=BlogPage />
                        <Route path=path!("/xfmail") view=XFMailPage />
                        <Route path=path!("/xfmail/demo") view=XFMailDemoPage />
                        <Route path=path!("/forum") view=|| view! { <div class="container py-20 text-center"><h1 class="text-3xl font-bold mb-4">"Forum"</h1><p class="text-gray-600">"Community forum coming soon."</p></div> } />
                    </Routes>
                </main>
                <Footer />
            </div>
        </Router>
    }
}
