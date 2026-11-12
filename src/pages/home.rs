use crate::components::MarkdownHtml;
use crate::content::ALL_PAGES;
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    let page = || ALL_PAGES.iter().find(|p| p.slug == "index");

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
                    <article>
                        <h1>"首页"</h1>
                        <p>"首页内容缺失：请在内容目录根部添加 index.md。"</p>
                    </article>
                }.into_any(),
            }}
        </div>
    }
}
