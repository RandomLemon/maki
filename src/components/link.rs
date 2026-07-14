use crate::content::BASE_URL;
use leptos::prelude::*;

fn full_href(href: &str) -> String {
    if href.starts_with('/') {
        format!("{}{}", BASE_URL.trim_end_matches('/'), href)
    } else {
        href.to_string()
    }
}

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
