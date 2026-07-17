use crate::components::Link;
use leptos::prelude::*;

/// 切换按钮点击事件:在 light/dark 之间翻转并持久化到 localStorage。
/// 图标显隐由 CSS 据 `html[data-theme]` 控制,脚本无需操作 DOM 文本。
const THEME_TOGGLE_SCRIPT: &str = "(function(){var doc=document.documentElement;var btn=document.getElementById('theme-toggle');if(!btn){return;}btn.addEventListener('click',function(){var cur=doc.getAttribute('data-theme');var next=cur==='dark'?'light':'dark';doc.setAttribute('data-theme',next);try{localStorage.setItem('maki-theme',next);}catch(e){}});})();";

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
                <button
                    type="button"
                    id="theme-toggle"
                    class="theme-toggle"
                    aria-label="切换浅色/深色模式"
                    title="切换浅色/深色模式"
                >
                    <svg
                        class="icon-moon"
                        viewBox="0 0 24 24"
                        width="18"
                        height="18"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        aria-hidden="true"
                    >
                        <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
                    </svg>
                    <svg
                        class="icon-sun"
                        viewBox="0 0 24 24"
                        width="18"
                        height="18"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        aria-hidden="true"
                    >
                        <circle cx="12" cy="12" r="4"></circle>
                        <line x1="12" y1="2" x2="12" y2="4"></line>
                        <line x1="12" y1="20" x2="12" y2="22"></line>
                        <line x1="4.93" y1="4.93" x2="6.34" y2="6.34"></line>
                        <line x1="17.66" y1="17.66" x2="19.07" y2="19.07"></line>
                        <line x1="2" y1="12" x2="4" y2="12"></line>
                        <line x1="20" y1="12" x2="22" y2="12"></line>
                        <line x1="4.93" y1="19.07" x2="6.34" y2="17.66"></line>
                        <line x1="17.66" y1="6.34" x2="19.07" y2="4.93"></line>
                    </svg>
                </button>
            </div>
            <script>{THEME_TOGGLE_SCRIPT}</script>
        </nav>
    }
}