---
title: 用 Leptos 做静态站点生成
date: 2024-02-20
tags: [leptos, rust, ssg]
summary: 记录使用 Leptos 0.7 构建纯静态博客的关键步骤与踩坑经验。
---

# 用 Leptos 做静态站点生成

Leptos 0.7 引入了更简洁的 SSR 渲染 API，使得不依赖 `cargo-leptos` 也能生成纯静态站点。

## 核心思路

1. 在编译时扫描 `content/` 目录。
2. 解析 Markdown + YAML frontmatter。
3. 使用 `RouteList::generate` 收集所有需要预渲染的路由。
4. 对每个路由调用 `.to_html()` 生成完整 HTML。

## 一个最小示例

```rust
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    location::RequestUrl,
    path,
    static_routes::StaticRoute,
    RouteList, SsrMode,
};
```

## 部署

生成后的 `dist/` 目录可以直接部署到 GitHub Pages、Cloudflare Pages 等任意静态托管服务。
