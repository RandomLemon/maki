use crate::components::Link;
use leptos::prelude::*;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="page not-found">
            <h1>"404"</h1>
            <p>"页面不存在。"</p>
            <Link href="/".to_string()>"返回首页"</Link>
        </div>
    }
}
