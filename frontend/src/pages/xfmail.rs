//! XFMail landing page

use leptos::prelude::*;
use leptos_router::components::A;

/// XFMail landing page with app description and demo link
#[component]
pub fn XFMailPage() -> impl IntoView {
    view! {
        <div class="page xfmail-page">
            // Hero Section
            <section class="hero hero-center">
                <div class="container">
                    <div class="hero-content text-center">
                        <div class="hero-icon mb-8">
                            <svg class="w-24 h-24 mx-auto text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
                                    d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"/>
                            </svg>
                        </div>
                        <h1 class="hero-title-bold">
                            "XFMail"
                        </h1>
                        <p class="hero-subtitle text-xl text-gray-600 max-w-2xl mx-auto mb-8">
                            "Offline-first email client with real-time collaboration, powered by CRDTs and the Braid protocol."
                        </p>
                        <div class="hero-cta flex justify-center gap-4">
                            <A href="/xfmail/demo" attr:class="btn btn-primary btn-lg">
                                "Try Demo"
                            </A>
                            <a href="https://github.com/xf-dev/xfmail/releases"
                               target="_blank"
                               rel="noopener noreferrer"
                               class="btn btn-secondary btn-lg">
                                "Download Desktop App"
                            </a>
                        </div>
                    </div>
                </div>
            </section>

            // Features Section
            <section class="section section-light">
                <div class="container">
                    <h2 class="text-3xl font-bold text-center mb-12">"Why XFMail?"</h2>
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                        // Feature 1: Offline-First
                        <div class="feature-card p-6 bg-white rounded-xl shadow-lg hover:shadow-xl transition-shadow">
                            <div class="feature-icon mb-4">
                                <svg class="w-12 h-12 text-blue-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                        d="M18.364 5.636a9 9 0 010 12.728m0 0l-2.829-2.829m2.829 2.829L21 21M15.536 8.464a5 5 0 010 7.072m0 0l-2.829-2.829m-4.243 2.829a4.978 4.978 0 01-1.414-2.83m-1.414 5.658a9 9 0 01-2.167-9.238m7.824 2.167a1 1 0 111.414 1.414m-1.414-1.414L3 3m8.293 8.293l1.414 1.414"/>
                                </svg>
                            </div>
                            <h3 class="text-xl font-semibold mb-2">"Offline-First"</h3>
                            <p class="text-gray-600">
                                "Work on your emails without internet. Everything syncs automatically when you're back online."
                            </p>
                        </div>

                        // Feature 2: Real-time Sync
                        <div class="feature-card p-6 bg-white rounded-xl shadow-lg hover:shadow-xl transition-shadow">
                            <div class="feature-icon mb-4">
                                <svg class="w-12 h-12 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                        d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>
                                </svg>
                            </div>
                            <h3 class="text-xl font-semibold mb-2">"Real-time Collaboration"</h3>
                            <p class="text-gray-600">
                                "Draft emails together. See changes instantly with CRDT-powered conflict-free editing."
                            </p>
                        </div>

                        // Feature 3: Native Performance
                        <div class="feature-card p-6 bg-white rounded-xl shadow-lg hover:shadow-xl transition-shadow">
                            <div class="feature-icon mb-4">
                                <svg class="w-12 h-12 text-purple-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                        d="M13 10V3L4 14h7v7l9-11h-7z"/>
                                </svg>
                            </div>
                            <h3 class="text-xl font-semibold mb-2">"Blazing Fast"</h3>
                            <p class="text-gray-600">
                                "Built with Rust and egui for native performance. Runs as a desktop app or in your browser."
                            </p>
                        </div>
                    </div>
                </div>
            </section>

            // Tech Stack Section
            <section class="section">
                <div class="container">
                    <h2 class="text-3xl font-bold text-center mb-8">"Built With"</h2>
                    <div class="flex flex-wrap justify-center gap-6">
                        <span class="tech-badge px-4 py-2 bg-orange-100 text-orange-800 rounded-full font-medium">"Rust"</span>
                        <span class="tech-badge px-4 py-2 bg-blue-100 text-blue-800 rounded-full font-medium">"egui"</span>
                        <span class="tech-badge px-4 py-2 bg-green-100 text-green-800 rounded-full font-medium">"Braid Protocol"</span>
                        <span class="tech-badge px-4 py-2 bg-purple-100 text-purple-800 rounded-full font-medium">"Diamond Types CRDT"</span>
                        <span class="tech-badge px-4 py-2 bg-red-100 text-red-800 rounded-full font-medium">"WebAssembly"</span>
                    </div>
                </div>
            </section>
        </div>
    }
}
