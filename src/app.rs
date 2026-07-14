use crate::components::{Footer, Navbar};
use crate::content::BASE_URL;
use crate::pages::{Archive, Home, NotFound, PostDetail, StaticPageView, TagPosts, Tags};
use leptos::prelude::*;
use leptos_router::{
    SsrMode,
    components::{Route, Router, Routes},
    path,
    static_routes::{StaticParamsMap, StaticRoute},
};

#[component]
pub fn App() -> impl IntoView {
    let router_base = if BASE_URL == "/" { "" } else { BASE_URL };
    let css_path = format!("{}/style.css", BASE_URL.trim_end_matches('/'));

    view! {
        <html lang="zh-CN">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
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
