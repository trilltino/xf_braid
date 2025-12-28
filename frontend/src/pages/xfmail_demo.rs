//! XFMail Demo page - embeds the egui WASM app

use leptos::prelude::*;
use leptos_router::components::A;

/// XFMail demo page that embeds the egui WASM application
#[component]
pub fn XFMailDemoPage() -> impl IntoView {
    view! {
        <div class="page xfmail-demo-page">
            // Header with back button
            <div class="demo-header bg-gray-900 text-white py-4">
                <div class="container flex items-center justify-between">
                    <A href="/xfmail" attr:class="flex items-center gap-2 text-gray-300 hover:text-white transition-colors">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"/>
                        </svg>
                        "Back to XFMail"
                    </A>
                    <h1 class="text-xl font-semibold">"XFMail Demo"</h1>
                    <a href="https://github.com/xf-dev/xfmail/releases"
                       target="_blank"
                       rel="noopener noreferrer"
                       class="text-sm text-gray-400 hover:text-white">
                        "Download App →"
                    </a>
                </div>
            </div>

            // Demo container with iframe
            <div class="demo-container" style="height: calc(100vh - 120px); background: #1a1a2e;">
                <iframe
                    src="/xfmail/index.html"
                    style="width:100%; height:100%; border:none;"
                    title="XFMail Demo"
                    allow="clipboard-read; clipboard-write"
                />
            </div>
        </div>
    }
}
