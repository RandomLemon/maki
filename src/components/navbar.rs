use crate::components::Link;
use leptos::prelude::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="navbar">
            <Link class="brand" href="/".to_string()>"Maki"</Link>
            <div class="nav-links">
                <Link href="/".to_string()>"首页"</Link>
                <Link href="/archive".to_string()>"文章目录"</Link>
                <Link href="/tags".to_string()>"标签"</Link>
                <Link href="/about".to_string()>"关于"</Link>
            </div>
        </nav>
    }
}
