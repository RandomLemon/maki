use crate::base::base_prefix;
use crate::components::{Footer, Navbar};
use crate::pages::{Archive, Home, NotFound, PostDetail, StaticPageView, TagPosts, Tags};
use leptos::prelude::*;
use leptos_router::{
    SsrMode,
    components::{Route, Router, Routes},
    path,
    static_routes::{StaticParamsMap, StaticRoute},
};

/// 在样式表应用前同步执行,根据 localStorage 或系统偏好设置主题,
/// 避免页面以默认浅色渲染后再切换造成的闪烁(FOUC)。
/// 未手动选择时还会监听系统主题变化,实现实时同步。
const THEME_INIT_SCRIPT: &str = "(function(){var doc=document.documentElement,key='maki-theme',saved=null;try{saved=localStorage.getItem(key);}catch(e){}function applyTheme(){var t=saved;if(t!=='light'&&t!=='dark'){t=matchMedia('(prefers-color-scheme: dark)').matches?'dark':'light';}doc.setAttribute('data-theme',t);}applyTheme();if(saved!=='light'&&saved!=='dark'){try{matchMedia('(prefers-color-scheme: dark)').addEventListener('change',applyTheme);}catch(e){}}})();";

#[component]
pub fn App() -> impl IntoView {
    let router_base = base_prefix();
    let css_path = format!("{}/style.css", router_base);

    view! {
        <html lang="zh-CN">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <script>{THEME_INIT_SCRIPT}</script>
                <title>"Maki"</title>
                <link rel="stylesheet" href=css_path />
            </head>
            <body>
                <Router base=router_base>
                    <Navbar />
                    <main>
                        <Routes fallback=|| view! { <NotFound /> }>
                            <Route
                                path=path!("/")
                                view=Home
                                ssr=SsrMode::Static(StaticRoute::new())
                            />
                            <Route
                                path=path!("/archive")
                                view=Archive
                                ssr=SsrMode::Static(StaticRoute::new())
                            />
                            <Route
                                path=path!("/tags")
                                view=Tags
                                ssr=SsrMode::Static(StaticRoute::new())
                            />
                            <Route
                                path=path!("/tag/:tag")
                                view=TagPosts
                                ssr=SsrMode::Static(
                                    StaticRoute::new().prerender_params(|| async move {
                                        let mut map = StaticParamsMap::new();
                                        let tags: std::collections::BTreeSet<String> =
                                            crate::content::ALL_POSTS
                                                .iter()
                                                .flat_map(|p| p.tags.iter().copied())
                                                .map(String::from)
                                                .collect();
                                        map.insert("tag".to_string(), tags.into_iter().collect());
                                        map
                                    })
                                )
                            />
                            <Route
                                path=path!("/posts/*slug")
                                view=PostDetail
                                ssr=SsrMode::Static(
                                    StaticRoute::new().prerender_params(|| async move {
                                        let mut map = StaticParamsMap::new();
                                        let slugs: Vec<String> = crate::content::ALL_POSTS
                                            .iter()
                                            .map(|p| p.slug.to_string())
                                            .collect();
                                        map.insert("slug".to_string(), slugs);
                                        map
                                    })
                                )
                            />
                            <Route
                                path=path!("/:slug")
                                view=StaticPageView
                                ssr=SsrMode::Static(
                                    StaticRoute::new().prerender_params(|| async move {
                                        let mut map = StaticParamsMap::new();
                                        let slugs: Vec<String> = crate::content::ALL_PAGES
                                            .iter()
                                            .map(|p| p.slug.to_string())
                                            .collect();
                                        map.insert("slug".to_string(), slugs);
                                        map
                                    })
                                )
                            />
                            <Route
                                path=path!("/*")
                                view=NotFound
                                ssr=SsrMode::Static(StaticRoute::new())
                            />
                        </Routes>
                    </main>
                    <Footer />
                </Router>
            </body>
        </html>
    }
}
