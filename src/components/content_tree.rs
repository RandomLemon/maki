use crate::components::Link;
use crate::models::ContentNode;
use leptos::prelude::*;

#[component]
pub fn ContentTree(node: &'static ContentNode) -> impl IntoView {
    view! {
        <ul class="content-tree">
            <For
                each=move || node.children
                key=|child| child.path
                let:child
            >
                <li>
                    {if child.is_post {
                        view! { <Link href=child.path.to_string()>{child.name}</Link> }.into_any()
                    } else {
                        view! {
                            <span class="folder">{child.name}</span>
                            <ContentTree node=child />
                        }.into_any()
                    }}
                </li>
            </For>
        </ul>
    }
}
