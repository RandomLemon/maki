use crate::components::Link;
use crate::content::ALL_POSTS;
use leptos::prelude::*;
use std::collections::BTreeSet;

#[component]
pub fn Tags() -> impl IntoView {
    let tags = move || {
        let mut set = BTreeSet::new();
        for post in ALL_POSTS {
            for tag in post.tags {
                set.insert(*tag);
            }
        }
        set.into_iter().collect::<Vec<_>>()
    };

    view! {
        <div class="page tags">
            <h1>"标签"</h1>
            <div class="tag-cloud">
                <For each=tags key=|tag| *tag let:tag>
                    <Link class="tag" href={format!("/tag/{}", tag)}>{tag}</Link>
                </For>
            </div>
        </div>
    }
}
