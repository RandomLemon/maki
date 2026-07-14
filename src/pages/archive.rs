use crate::components::ContentTree;
use crate::content::CONTENT_TREE;
use leptos::prelude::*;

#[component]
pub fn Archive() -> impl IntoView {
    view! {
        <div class="page archive">
            <h1>"文章目录"</h1>
            <ContentTree node=CONTENT_TREE />
        </div>
    }
}
