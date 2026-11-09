# AGENTS.md

本文件面向在本仓库工作的编码代理。开始改动前先读 `docs/README.md`，再按任务阅读对应文档。文档描述源码行为；若源码与文档冲突，以源码为准，并在改动中同步修正文档。

## 项目定位

Maki 是一个 **Rust 2024 + Leptos 0.7(SSR)** 的静态博客生成器：

- `build.rs` 在编译期扫描 Markdown、解析 frontmatter、生成 `OUT_DIR/content.rs`。
- `src/main.rs` 枚举静态路由，调用 `App().to_html()`，把 HTML 写入 `dist/`。
- 运行时不依赖 WASM、hydration、客户端路由或后端服务；浏览器端只有主题切换用的内联 JavaScript。
- 不要引入 `cargo-leptos`、hydration 或新的运行时框架，除非任务明确要求。

## 环境与命令

推荐使用 Nix flake：

```bash
nix develop       # 或 direnv allow
cargo run         # 生成站点到 dist/
```

| 命令 | 用途 |
| --- | --- |
| `cargo check` | 类型检查，不生成站点 |
| `cargo run` | 构建并生成默认根路径站点 |
| `MAKI_BASE_URL=/maki cargo run` | 生成子路径部署版本 |
| `cargo run --release` | release 构建并生成 |
| `caddy file-server --listen :8080 --browse --root dist` | 本地预览 |
| `cargo clean -p maki` | 清理本包及 `build.rs` 输出 |

仓库目前**没有自动化测试套件**。不要声称“测试通过”；用实际构建和产物检查验证，见下文“验证”。

## 架构速查

| 文件/目录 | 职责 |
| --- | --- |
| `build.rs` | 内容扫描、frontmatter 解析、Markdown/代码高亮、生成 `content.rs` |
| `src/main.rs` | 清理 `dist/`、枚举路由、写 HTML、合并 CSS、复制资源、404/重定向 |
| `src/app.rs` | 根组件、路由表、主题初始化脚本 |
| `src/base.rs` | `base_prefix()` 与 `full_href()` |
| `src/content.rs` | `include!(concat!(env!("OUT_DIR"), "/content.rs"))` |
| `src/models/post.rs` | `Post`、`StaticPage`、`ContentNode` |
| `src/components/` | `Navbar`、`Footer`、`Link`、`MarkdownHtml`、`ContentTree` |
| `src/pages/` | 各路由页面组件 |
| `public/styles/` | 合并为 `style.css` 的样式源文件 |
| `content/` | 默认内容目录，可用 `MAKI_CONTENT_DIR` 覆盖 |

详细说明：[architecture.md](docs/architecture.md)。

## 硬性约束

1. **内容在编译期嵌入。** 修改 `content/` 后必须重新 `cargo run`；页面组件中不要读取文件或访问网络。
2. **不要手改生成物。** `dist/` 每次生成都会整个删除；`OUT_DIR/content.rs` 由 `build.rs` 生成。改源头，不要改结果。
3. **站内绝对链接必须带 base。** 组件中用 `Link` 或 `full_href("/path")`；不要硬编码 `href="/path"`。Markdown 正文中的链接不会被自动改写。
4. **路由必须可静态枚举。** 新路由使用 `SsrMode::Static(...)`；带参数的路由必须提供 `prerender_params` 枚举全部取值，否则不会产出页面。
5. **新增样式必须登记。** 在 `public/styles/` 新建 CSS 后，把文件名加入 `src/main.rs` 的 `merge_styles()` 顺序数组，否则不会进入 `style.css`。
6. **页面来自内容目录根部。** 独立页面是内容目录根部的 `.md` 文件（如 `content/about.md` → `/about`）；扫描不递归子目录，且 `/:slug` 只匹配单段路径。
7. **非 ASCII 内容显式写 `summary`（尤其中文）。** 未写摘要时按 200 **字节**截断，可能切在多字节字符中间并导致构建 panic。
8. **HTML 视为可信内容。** `MarkdownHtml` 用 `inner_html` 注入预生成 HTML，Markdown 原始 HTML 也会直出；不要接入未经审核的第三方内容。
9. **`dist/` 是临时目录。** 每次生成会删除整个 `dist/`，不要把需要持久化的文件放进去。

## 常见改动

### 新增页面路由

1. 在 `src/pages/` 新建组件。
2. 在 `src/pages/mod.rs` 中 `pub mod` 并 `pub use`。
3. 在 `src/app.rs` 的 `Routes` 中注册 `SsrMode::Static(StaticRoute::new())`。
4. 如需入口，用 `Link` 添加导航或页面链接。
5. 顶层单段路径必须显式注册；否则会被 `/:slug` 捕获并显示“页面不存在”。

### 新增带参数路由

参考 `src/app.rs` 中的 `/tag/:tag`、`/posts/*slug`、`/:slug`：

```rust
StaticRoute::new().prerender_params(|| async move {
    let mut map = StaticParamsMap::new();
    map.insert("tag".to_string(), vec!["rust".to_string()]);
    map
})
```

参数值必须覆盖所有需要生成的页面。

### 修改内容字段

1. 改 `build.rs` 中的 `PostMeta`/`PageMeta` 和内部结构体。
2. 改 `src/models/post.rs` 中的公开结构体。
3. 改 `build.rs::generate_content_rs()`，把新字段写进生成代码。
4. 更新 `docs/content.md` 的字段表。

### 修改样式

按 `src/main.rs` 中 `merge_styles()` 的顺序登记文件。顺序影响层叠，`variables.css` 应保持在最前。

## 验证

至少执行：

```bash
cargo check
cargo run
```

然后检查关键产物：

```bash
test -f dist/index.html
test -f dist/404.html
test -f dist/style.css
test -f dist/posts/rust/memory-layout/index.html
grep -o 'href="[^"]*"' dist/index.html | head
```

涉及 base、导航、静态资源或部署的改动，必须再验证子路径模式：

```bash
MAKI_BASE_URL=/maki cargo run
test -f dist/maki/index.html
test -f dist/maki/404.html
test -f dist/index.html
grep -o 'href="[^"]*"' dist/maki/index.html | head
cargo run   # 恢复默认根路径构建
```

需要视觉检查时启动实际服务器并打开页面：

```bash
caddy file-server --listen :8080 --browse --root dist
```

重点确认导航、文章页、标签页、归档树、深色模式首屏无闪烁、404 页面样式。只编译通过不算完成；必须实际生成并检查产物。

## 已知限制

以下行为是当前实现的已知状态，详见 `docs/development.md`。不要在没有任务要求时顺手改动其语义；若确实修复，同步更新文档和验证。

- 首页无分页，直接列出全部文章。
- 无 RSS、sitemap、搜索。
- 所有页面 `<title>` 固定为 `Maki`。
- 无客户端 hydration；交互仅限内联主题脚本。
- 独立页面只扫描内容目录根部的 Markdown；子目录中的 Markdown 不会生成页面。
- 自动摘要按字节截断，可能 panic。
- 正文若以 `# 标题` 开头，会与模板 `<h1>` 重复。
- Markdown 正文中的站内绝对链接不会自动加 base。
- 标签原样拼入 `/tag/<tag>`，未做百分号编码。
- 默认 GitHub Pages workflow 只上传 `dist/maki/`，因此 `dist/index.html` 跳转页、`dist/.nojekyll`、`dist/CNAME` 不会进入 artifact。

## 文档同步

修改以下内容时更新对应文档：

| 变更 | 更新 |
| --- | --- |
| 内容字段、Markdown 规则、摘要、排序 | `docs/content.md` |
| 路由表、构建流程、数据模型、主题机制 | `docs/architecture.md` |
| 环境变量、输出结构、CSS 合并、部署、故障排查 | `docs/configuration.md` |
| 开发流程、扩展方式、验证、已知限制 | `docs/development.md` |

文档入口：`docs/README.md`。改动后不要留下未验证的文档声明。
