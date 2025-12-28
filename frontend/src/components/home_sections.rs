use leptos::prelude::*;
use leptos_router::components::A;

/// Section showing apps built with XFBraid
#[component]
pub fn AppsShowcase() -> impl IntoView {
    // Placeholder data for apps
    let apps = vec![
        ("Inkspace", "bg-gray-800", "✒️"),
        ("Evolution", "bg-blue-600", "📧"),
        ("Image Viewer", "bg-orange-500", "🖼️"),
        ("Polari", "bg-green-500", "💬"),
        ("Podcasts", "bg-red-500", "🎙️"),
        ("Maps", "bg-yellow-500", "🗺️"),
    ];

    view! {
        <section class="section section-light bg-white">
            <div class="container text-center">
                <h2 class="text-4xl font-light mb-4">"Apps built with XFBraid"</h2>
                <p class="text-xl text-gray-500 mb-16 max-w-2xl mx-auto">
                    "Developers around the world have used XFBraid as a platform to create apps that solve problems faced by end-users."
                </p>

                <div class="flex flex-wrap justify-center gap-12 md:gap-16">
                    {apps.into_iter().map(|(name, color, icon)| {
                        view! {
                            <div class="flex flex-col items-center gap-4 group cursor-pointer transition-transform hover:-translate-y-1">
                                <div class={format!("w-24 h-24 rounded-2xl shadow-lg flex items-center justify-center text-4xl text-white transform transition-transform group-hover:scale-105 {}", color)}>
                                    <span>{icon}</span>
                                </div>
                                <span class="text-gray-600 font-medium">{name}</span>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>
    }
}

/// Feature grid with orange icons
#[component]
pub fn FeatureGrid() -> impl IntoView {
    let features = vec![
        ("Portability", "Projects built using XFBraid and its dependencies run on well known operating systems.", "🏗️"),
        ("Stability", "XFBraid delivers the enticing features and superb performance which adds to your applications.", "⚖️"),
        ("Language Bindings", "XFBraid is written in Rust but is designed to support a wide range of languages.", "🔀"),
        ("Interfaces", "XFBraid has a comprehensive collection of core widgets like Buttons, Windows, Toolbars.", "🧩"),
        ("Open Source", "XFBraid is a free and open-source project maintained by XFTamapa and an active community.", "🔓"),
        ("API", "XFBraid boasts of an easy to use API which helps in decreasing your development time.", "🏷️"),
        ("Accommodation", "XFBraid caters to many features like Native look and feel, theme support, Object-oriented approach.", "🧳"),
        ("Foundations", "XFBraid is built on top of robust async runtimes and system integration points.", "📚"),
    ];

    view! {
        <section class="section bg-white pt-24 pb-24">
            <div class="container">
                <h2 class="text-4xl font-light mb-16 text-center">"A feature-rich development tool"</h2>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-x-12 gap-y-12">
                    {features.into_iter().map(|(title, desc, icon)| {
                        view! {
                            <div class="flex gap-6 items-start">
                                <div class="flex-shrink-0 w-12 h-12 rounded-full bg-[#e66100] flex items-center justify-center text-white text-xl">
                                    {icon}
                                </div>
                                <div>
                                    <h3 class="text-xl text-gray-800 font-normal mb-2">{title}</h3>
                                    <p class="text-gray-500 leading-relaxed font-light">{desc}</p>
                                </div>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>
    }
}

/// Info cards for developers and community
#[component]
pub fn InfoCards() -> impl IntoView {
    view! {
        <section class="section section-light bg-[#f8f9fa] border-t border-gray-200">
            <div class="container">
                // Top Row
                <div class="grid grid-cols-1 md:grid-cols-3 gap-8 mb-24">
                    <div class="info-card">
                        <div class="flex justify-between items-start mb-4">
                            <h3 class="text-2xl font-light text-gray-800">"Develop with XFBraid"</h3>
                            <span class="text-2xl text-gray-400">"💼"</span>
                        </div>
                        <p class="text-gray-500 mb-6 leading-relaxed">
                            "By taking advantage of XFBraid being a cross-platform tool, you can develop amazing apps using the API. If you are interested in developing an app, get started now."
                        </p>
                        <A href="/docs" attr:class="text-blue-600 hover:text-blue-800 hover:underline">"exmaple application"</A>
                    </div>

                    <div class="info-card">
                        <div class="flex justify-between items-start mb-4">
                            <h3 class="text-2xl font-light text-gray-800">"Develop XFBraid"</h3>
                            <span class="text-2xl text-gray-400">"⚙️"</span>
                        </div>
                        <p class="text-gray-500 mb-6 leading-relaxed">
                            "XFBraid is a large project and relies on volunteers. To help us with the project development, hack away on the existing bugs and feature requests."
                        </p>
                        <A href="/contribute" attr:class="text-blue-600 hover:text-blue-800 hover:underline">"bugs and feature requests"</A>
                    </div>

                    <div class="info-card">
                        <div class="flex justify-between items-start mb-4">
                            <h3 class="text-2xl font-light text-gray-800">"Looking for Help?"</h3>
                            <span class="text-2xl text-gray-400">"❓"</span>
                        </div>
                        <p class="text-gray-500 mb-6 leading-relaxed">
                            "If you want to ask questions about XFBraid, whether it's for developing apps or contributing, you can use our Discourse instance or Matrix room."
                        </p>
                        <A href="/help" attr:class="text-blue-600 hover:text-blue-800 hover:underline">"Matrix room"</A>
                    </div>
                </div>

                // News and Events Header
                <div class="text-center mb-12">
                    <h2 class="text-4xl font-light">"News and Events"</h2>
                </div>

                // Bottom Row
                <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                     <div class="info-card">
                        <h3 class="text-xl text-gray-800 font-normal mb-4 border-b pb-4 border-gray-100">"Catch up with dev"</h3>
                        <p class="text-gray-500 mb-4 leading-relaxed">
                            "Get in touch with developers through Matrix. Get updates about XFBraid and its community from the blog."
                        </p>
                        <A href="/blog" attr:class="text-blue-600 hover:text-blue-800 hover:underline">"XFBraid blog"</A>
                    </div>

                    <div class="info-card">
                        <h3 class="text-xl text-gray-800 font-normal mb-4 border-b pb-4 border-gray-100">"Meet the community"</h3>
                        <p class="text-gray-500 mb-4 leading-relaxed">
                            "Regular team meetings take place at conferences and hackfests to discuss the future of XFBraid and define a roadmap."
                        </p>
                        <A href="/community" attr:class="text-blue-600 hover:text-blue-800 hover:underline">"roadmap"</A>
                    </div>

                    <div class="info-card">
                        <h3 class="text-xl text-gray-800 font-normal mb-4 border-b pb-4 border-gray-100">"Contribute"</h3>
                        <p class="text-gray-500 mb-4 leading-relaxed">
                            "If you are a developer and want to contribute to XFBraid, you are more than welcome to do so."
                        </p>
                        <A href="/contribute" attr:class="text-blue-600 hover:text-blue-800 hover:underline">"welcome to do so"</A>
                    </div>
                </div>
            </div>
        </section>
    }
}
