use crate::components::Link;
use crate::content::ALL_POSTS;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn TagPosts() -> impl IntoView {
    let params = use_params_map();
    let tag = move || params.read().get("tag").unwrap_or_default();

    let posts = move || {
        let t = tag();
        ALL_POSTS
            .iter()
            .filter(|p| p.tags.contains(&t.as_str()))
            .collect::<Vec<_>>()
    };

    view! {
        <div class="page tag-posts">
            <h1>{move || format!("标签: {}", tag())}</h1>
            <div class="post-list">
                <For each=posts key=|post| post.slug let:post>
                    <article class="post-card">
                        <h2>
                            <Link href=post.full_path.to_string()>{post.title}</Link>
                        </h2>
                        <div class="meta">
                            <span>{post.date.to_string()}</span>
                        </div>
                        <p class="summary">{post.summary}</p>
                    </article>
                </For>
            </div>
        </div>
    }
}
