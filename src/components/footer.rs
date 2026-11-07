use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="footer">
            <p>"Powered by "<a href="https://github.com/RandomLemon/maki">"Maki"</a></p>
        </footer>
    }
}
