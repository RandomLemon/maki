use crate::components::MarkdownHtml;
use crate::content::ALL_PAGES;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn StaticPageView() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.read().get("slug").unwrap_or_default();

    let page = move || {
        let s = slug();
        ALL_PAGES.iter().find(|p| p.slug == s)
    };

    view! {
        <div class="page static-page">
            {move || match page() {
                Some(page) => view! {
                    <article>
                        <h1>{page.title}</h1>
                        <MarkdownHtml html=page.content_html />
                    </article>
                }.into_any(),
                None => view! {
                    <p>"页面不存在。"</p>
                }.into_any(),
            }}
        </div>
    }
}
