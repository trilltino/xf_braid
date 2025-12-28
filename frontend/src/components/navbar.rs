//! Navigation bar component

use leptos::prelude::*;
use leptos_router::components::A;

/// Navigation bar
#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="navbar">
            <div class="navbar-brand">
                <A href="/"><strong>"XFBraid"</strong></A>
            </div>

            <div class="navbar-menu">
                <A href="/docs">"Docs"</A>
                <A href="/community">"Community"</A>
                <A href="/xfmail">"XFMail"</A>
                <A href="/forum">"Forum"</A>
                <A href="/blog">"Blog"</A>
            </div>
        </nav>
    }
}
