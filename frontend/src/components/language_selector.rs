use leptos::prelude::*;

#[component]
pub fn LanguageSelector() -> impl IntoView {
    let (active_tab, set_active_tab) = signal("Rust".to_string());

    let tabs = vec!["Rust", "C++", "Python", "JavaScript", "Haskell"];

    view! {
        <div class="lang-selector">
            // Tab Header
            <div class="lang-tabs">
                {tabs.into_iter().map(|lang| {
                    let lang_str = lang.to_string();
                    let lang_for_active = lang_str.clone();
                    let lang_for_click = lang_str.clone();

                    let is_active = move || active_tab.get() == lang_for_active;
                    view! {
                        <button
                            class="lang-tab"
                            class:active=is_active
                            on:click=move |_| set_active_tab.set(lang_for_click.clone())
                        >
                            {lang_str}
                        </button>
                    }
                }).collect_view()}
            </div>

            // Code Display
            <div class="lang-code-window">
                {move || {
                    let current = active_tab.get();
                    match current.as_str() {
                        "Rust" => view! {
                            <div class="code-content">
                                <pre><code>
                                    <span class="hl-comment">"// Include xfbraid"</span><br/>
                                    <span class="hl-keyword">"use"</span> " xfbraid::prelude::*;"<br/>
                                    <br/>
                                    <span class="hl-keyword">"fn"</span> <span class="hl-func">" main"</span> "() {"<br/>
                                    "    "<span class="hl-comment">"// Create a new application"</span><br/>
                                    "    "<span class="hl-keyword">"let"</span> " app = App::"<span class="hl-func">"new"</span>"("<span class="hl-string">"\"org.example.App\""</span>");"<br/>
                                    <br/>
                                    "    "<span class="hl-comment">"// Connect signals"</span><br/>
                                    "    app."<span class="hl-func">"connect_activate"</span>"(|app| {"<br/>
                                    "        "<span class="hl-keyword">"let"</span> " win = ApplicationWindow::"<span class="hl-func">"new"</span>"(app);"<br/>
                                    "        win."<span class="hl-func">"set_title"</span>"("<span class="hl-string">"\"Hello World!\""</span>");"<br/>
                                    "        win."<span class="hl-func">"present"</span>"();"<br/>
                                    "    });"<br/>
                                    "}"
                                </code></pre>
                            </div>
                        }.into_any(),
                        "C++" => view! {
                             <div class="code-content">
                                <pre><code>
                                    <span class="hl-comment">"// Include xfbraid"</span><br/>
                                    <span class="hl-keyword">"#include"</span> <span class="hl-string">"&lt;xfbraid/app.h&gt;"</span><br/>
                                    <br/>
                                    <span class="hl-keyword">"int"</span> <span class="hl-func">" main"</span> "(int argc, char *argv[]) {"<br/>
                                    "    "<span class="hl-comment">"// Create a new application"</span><br/>
                                    "    "<span class="hl-keyword">"auto"</span> " app = xfbraid::App::"<span class="hl-func">"create"</span>"("<span class="hl-string">"\"org.example.App\""</span>");"<br/>
                                    <br/>
                                    "    "<span class="hl-keyword">"return"</span> " app-&gt;"<span class="hl-func">"run"</span>"(argc, argv);"<br/>
                                    "}"
                                </code></pre>
                            </div>
                        }.into_any(),
                        "Python" => view! {
                             <div class="code-content">
                                <pre><code>
                                    <span class="hl-comment">"# Include xfbraid"</span><br/>
                                    <span class="hl-keyword">"import"</span> " sys"<br/>
                                    <span class="hl-keyword">"import"</span> " xfbraid"<br/>
                                    <br/>
                                    <span class="hl-keyword">"class"</span> <span class="hl-func">" MyApp"</span>"(xfbraid.Application):"<br/>
                                    "    "<span class="hl-keyword">"def"</span> <span class="hl-func">" do_activate"</span>"(self):"<br/>
                                    "        "<span class="hl-comment">"# Create a new window"</span><br/>
                                    "        win = xfbraid.ApplicationWindow(application=self)"<br/>
                                    "        win.set_title("<span class="hl-string">"\"Hello World!\""</span>")"<br/>
                                    "        win.present()"<br/>
                                    <br/>
                                    "app = MyApp(application_id="<span class="hl-string">"\"org.example.App\""</span>")"<br/>
                                    "app.run(sys.argv)"
                                </code></pre>
                            </div>
                        }.into_any(),
                        "JavaScript" => view! {
                             <div class="code-content">
                                <pre><code>
                                    <span class="hl-comment">"// Include xfbraid"</span><br/>
                                    <span class="hl-keyword">"import"</span> " xfbraid " <span class="hl-keyword">"from"</span> <span class="hl-string">" 'xfbraid'"</span>";"<br/>
                                    <br/>
                                    <span class="hl-keyword">"const"</span> " app = "<span class="hl-keyword">"new"</span>" xfbraid.App({"<br/>
                                    "    applicationId: "<span class="hl-string">"\"org.example.App\""</span><br/>
                                    "});"<br/>
                                    <br/>
                                    "app."<span class="hl-func">"on"</span>"("<span class="hl-string">"\"activate\""</span>", () =&gt; {"<br/>
                                    "    "<span class="hl-keyword">"const"</span> " win = "<span class="hl-keyword">"new"</span>" xfbraid.Window(app);"<br/>
                                    "    win.present();"<br/>
                                    "});"<br/>
                                    <br/>
                                    "app.run(ARGV);"
                                </code></pre>
                            </div>
                        }.into_any(),
                        "Haskell" => view! {
                             <div class="code-content">
                                <pre><code>
                                    <span class="hl-comment">"-- Include xfbraid"</span><br/>
                                    <span class="hl-keyword">"import"</span> " GI.XFBraid"<br/>
                                    <br/>
                                    <span class="hl-keyword">"main"</span> <span class="hl-func">" :: IO ()"</span><br/>
                                    <span class="hl-func">"main"</span> " = do"<br/>
                                    "    "<span class="hl-comment">"-- Create a new application"</span><br/>
                                    "    app &lt;- appNew "<span class="hl-string">"\"org.example.App\""</span><br/>
                                    <br/>
                                    "    "<span class="hl-comment">"-- Connect signals"</span><br/>
                                    "    on app #activate $ \\_ -&gt; do"<br/>
                                    "        win &lt;- appWindowNew app"<br/>
                                    "        setWinTitle win "<span class="hl-string">"\"Hello World!\""</span><br/>
                                    "        windowPresent win"<br/>
                                    <br/>
                                    "    appRun app"
                                </code></pre>
                            </div>
                        }.into_any(),
                        _ => view! { <div>"Select a language"</div> }.into_any()
                    }
                }}
            </div>
        </div>
    }
}
