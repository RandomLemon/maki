use crate::components::{Link, MarkdownHtml};
use crate::content::ALL_POSTS;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn PostDetail() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.read().get("slug").unwrap_or_default();

    let post = move || {
        let s = slug();
        ALL_POSTS.iter().find(|p| p.slug == s)
    };

    view! {
        <div class="page post-detail">
            {move || match post() {
                Some(post) => view! {
                    <article>
                        <header>
                            <h1>{post.title}</h1>
                            <div class="meta">
                                <span>{post.date.to_string()}</span>
                                <span class="tags">
                                    <For each=move || post.tags key=|tag| *tag let:tag>
                                        <Link class="tag" href={format!("/tag/{}", tag)}>{*tag}</Link>
                                    </For>
                                </span>
                            </div>
                        </header>
                        <MarkdownHtml html=post.content_html />
                    </article>
                }.into_any(),
                None => view! {
                    <p>"文章不存在。"</p>
                }.into_any(),
            }}
        </div>
    }
}
