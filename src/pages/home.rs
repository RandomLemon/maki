use crate::components::Link;
use crate::content::ALL_POSTS;
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="page home">
            <h1>"最新文章"</h1>
            <div class="post-list">
                <For
                    each=|| ALL_POSTS
                    key=|post| post.slug
                    let:post
                >
                    <article class="post-card">
                        <h2>
                            <Link href=post.full_path.to_string()>{post.title}</Link>
                        </h2>
                        <div class="meta">
                            <span>{post.date.to_string()}</span>
                            <span class="tags">
                                <For each=move || post.tags key=|tag| *tag let:tag>
                                    <Link class="tag" href={format!("/tag/{}", tag)}>{*tag}</Link>
                                </For>
                            </span>
                        </div>
                        <p class="summary">{post.summary}</p>
                    </article>
                </For>
            </div>
        </div>
    }
}
