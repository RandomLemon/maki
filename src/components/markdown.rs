use leptos::prelude::*;

#[component]
pub fn MarkdownHtml(html: &'static str) -> impl IntoView {
    view! {
        <div class="markdown-body" inner_html=html />
    }
}
