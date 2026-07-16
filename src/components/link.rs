use crate::base::full_href;
use leptos::prelude::*;

#[component]
pub fn Link(
    href: String,
    #[prop(optional)] class: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let full = full_href(&href);
    match class {
        Some(class) => view! { <a href=full class=class>{children()}</a> }.into_any(),
        None => view! { <a href=full>{children()}</a> }.into_any(),
    }
}