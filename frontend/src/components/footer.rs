use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Footer() -> impl IntoView {
    let current_year = 2025; // In a real app, you might want to get this dynamically

    view! {
        <footer class="footer">
            <div class="container footer-content">
                // Left Brand Section
                <div class="footer-brand-col">
                    <div class="footer-logo">
                        <span class="logo-icon">"⋈"</span> // Placeholder icon
                        <span class="logo-text">"XFBraid"</span>
                    </div>
                    <p class="footer-text">
                        "XFBraid is maintained by XFSolutions."
                    </p>
                    <p class="footer-copyright">
                        "© " {current_year} " XFSolutions. All Rights Reserved."
                    </p>
                </div>

                // Links Columns
                <div class="footer-links-grid">
                    // Project Column
                    <div class="footer-col">
                        <h4 class="footer-heading">"PROJECT"</h4>
                        <ul class="footer-list">
                            <li><A href="/features" attr:class="footer-link">"Features"</A></li>
                            <li><A href="/docs" attr:class="footer-link">"Docs"</A></li>
                            <li><A href="/downloads" attr:class="footer-link">"Downloads"</A></li>
                            <li><A href="/license" attr:class="footer-link">"License"</A></li>
                        </ul>
                    </div>

                    // Support Column
                    <div class="footer-col">
                        <h4 class="footer-heading">"SUPPORT"</h4>
                        <ul class="footer-list">
                            <li><A href="/faq" attr:class="footer-link">"FAQs"</A></li>
                            <li><A href="/bugs" attr:class="footer-link">"Report a Bug"</A></li>
                            <li><A href="/request" attr:class="footer-link">"Request a feature"</A></li>
                        </ul>
                    </div>

                    // Community Column
                    <div class="footer-col">
                        <h4 class="footer-heading">"COMMUNITY"</h4>
                        <ul class="footer-list">
                            <li><A href="/community" attr:class="footer-link">"Community"</A></li>
                            <li><A href="/blog" attr:class="footer-link">"Development Blog"</A></li>
                            <li><A href="/chat" attr:class="footer-link">"Matrix"</A></li>
                        </ul>
                    </div>

                    // About Column
                    <div class="footer-col">
                        <h4 class="footer-heading">"About the Site"</h4>
                        <ul class="footer-list">
                            <li class="footer-text-item">"Hosted on " <a href="#" class="link-plain">"Fly.io"</a> "."</li>
                            <li class="footer-text-item">"Powered by " <a href="https://leptos.dev" class="link-plain">"Leptos"</a> "."</li>
                            <li class="footer-text-item"><a href="#" class="link-plain">"Report an issue"</a> "."</li>
                        </ul>
                        // Social Icons Placeholder
                        <div class="footer-socials">
                            <span class="social-icon">"GH"</span>
                            <span class="social-icon">"DC"</span>
                            <span class="social-icon">"TW"</span>
                        </div>
                    </div>
                </div>
            </div>
        </footer>
    }
}
