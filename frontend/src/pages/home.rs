//! Home page component

use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::{AppsShowcase, FeatureGrid, InfoCards, LanguageSelector};

/// Landing page
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="page home-page">
            <section class="hero hero-left">
                <div class="container">
                    <div class="hero-content">
                        <h1 class="hero-title-bold">
                            "Apps that work."<br/>
                            "Code that scales."
                        </h1>
                        <p class="hero-description">
                            "Explore a curated collection of Braid examples—from lightweight utilities to full-featured collaborative applications."
                        </p>
                        <div class="hero-cta">
                            <A href="/docs" attr:class="btn btn-primary">"Get Started"</A>
                            <A href="/features" attr:class="btn btn-secondary">"Learn More"</A>
                        </div>
                    </div>
                </div>
            </section>

            <section class="section section-light">
                <div class="container">
                    <div class="text-center mb-12">
                        <h2 class="text-4xl font-light mb-6">"Work with the language of your choice"</h2>
                        <p class="text-xl text-gray-600 max-w-3xl mx-auto leading-relaxed">
                            "Develop your App with your language of choice by using Language Bindings or wrappers and take full advantage of the official Bindings which guarantee API stability and time-based releases."
                        </p>
                    </div>

                    <LanguageSelector />
                </div>
            </section>

            <AppsShowcase />
            <FeatureGrid />
            <InfoCards />
        </div>
    }
}
