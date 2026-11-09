# 开发指南

## 环境准备

推荐使用仓库自带的 Nix flake，`flake.nix` 提供：

- `rust`（由 `rust-toolchain.toml` 固定的 stable 工具链）
- `pkg-config`
- `openssl.dev`
- `clang`
- `caddy`

```bash
# 方式一:显式进入开发环境
nix develop

# 方式二:使用 direnv(项目根已有 .envrc: use flake)
direnv allow
```

也可以直接使用系统 Rust：

```bash
cargo --version
cargo run
```

项目使用 Rust 2024 edition，`rust-toolchain.toml` 指定 stable 通道。若依赖编译时缺少系统库或 C 工具链，优先回到 `nix develop` 环境。

## 常用命令

| 命令 | 作用 |
| --- | --- |
| `cargo run` | 构建并生成站点到 `dist/` |
| `cargo run --release` | release 构建并生成站点 |
| `cargo check` | 只做类型检查，不生成站点 |
| `cargo build` | 构建二进制，不运行生成器 |
| `caddy file-server --listen :8080 --browse --root dist` | 本地预览 |
| `MAKI_BASE_URL=/maki cargo run` | 生成子路径部署版本 |
| `cargo clean -p maki` | 清理本包构建产物（含 `build.rs` 输出） |

生成器运行后会打印每个输出文件和最终目录；没有独立的测试命令。

## 源码职责

| 文件 | 职责 |
| --- | --- |
| `build.rs` | 扫描内容、解析 frontmatter、Markdown 转 HTML、代码高亮、生成 `OUT_DIR/content.rs` |
| `src/main.rs` | 静态生成器入口：清理输出、枚举路由、渲染 HTML、合并 CSS、复制资源、写 404/重定向 |
| `src/app.rs` | 根组件、路由声明、主题初始化脚本 |
| `src/base.rs` | `base_prefix()`、`full_href()` |
| `src/content.rs` | `include!(OUT_DIR/content.rs)` |
| `src/models/post.rs` | `Post`、`StaticPage`、`ContentNode` 数据结构 |
| `src/components/` | 可复用 UI 组件 |
| `src/pages/` | 路由页面组件 |
| `public/styles/` | 样式源文件，构建时合并 |
| `public/.nojekyll` | 部署到 artifact 根级 |

## 扩展方式

### 新增一个页面路由

1. 在 `src/pages/` 新建组件，例如 `notes.rs`：

```rust
use leptos::prelude::*;

#[component]
pub fn Notes() -> impl IntoView {
    view! {
        <div class="page notes">
            <h1>"笔记"</h1>
        </div>
    }
}
```

2. 在 `src/pages/mod.rs` 中声明并导出：

```rust
pub mod notes;

pub use notes::Notes;
```

3. 在 `src/app.rs` 的 `Routes` 中注册静态路由：

```rust
<Route
    path=path!("/notes")
    view=Notes
    ssr=SsrMode::Static(StaticRoute::new())
/>
```

4. 在 `Home`、`Navbar` 等需要入口的地方用 `Link` 添加链接。

注意：

- 必须使用 `SsrMode::Static`，否则 `RouteList::generate` 不会把它加入生成列表。
- 新增路径含参数时，必须提供 `prerender_params` 返回所有可能取值，否则不会生成对应页面。
- 顶层单段路径要显式注册，否则会被 `/:slug` 捕获并落入 `StaticPageView`。

### 新增带参数路由

参考 `src/app.rs` 中 `/tag/:tag`、`/posts/*slug`、`/:slug` 的写法：

```rust
<Route
    path=path!("/tag/:tag")
    view=TagPosts
    ssr=SsrMode::Static(
        StaticRoute::new().prerender_params(|| async move {
            let mut map = StaticParamsMap::new();
            // 这里必须枚举所有取值
            map.insert("tag".to_string(), vec!["rust".to_string()]);
            map
        })
    )
/>
```

生成器只会输出 `into_static_paths()` 返回的路径；没有参数的“动态路由”不会在静态站点中存在。

### 新增组件

1. 在 `src/components/` 新建文件。
2. 在 `src/components/mod.rs` 中 `pub mod` 并 `pub use`。
3. 在页面中通过 `use crate::components::...` 引入。

站内链接优先使用 `Link` 组件：

```rust
<Link href="/archive".to_string()>"文章目录"</Link>
```

`Link` 会对以 `/` 开头的 href 调用 `full_href()`，自动加 base 前缀；外部链接与相对链接原样输出。

### 新增样式文件

1. 放到 `public/styles/`。
2. 在 `src/main.rs` 的 `merge_styles()` 顺序数组中登记。

```rust
let order = [
    "variables.css",
    "base.css",
    "layout.css",
    "navbar.css",
    "post.css",
    "markdown.css",
    "syntax.css",
    "archive.css",
    "tags.css",
    "footer.css",
    "not-found.css",
    // 新文件加在这里
];
```

未登记的文件不会进入 `dist/style.css`。顺序会影响层叠，变量文件应保持在最前。

### 修改内容数据结构

如果要新增文章/页面字段：

1. 修改 `build.rs` 中的 `PostMeta` / `PageMeta` 和内部 `Post` / `StaticPage`。
2. 修改 `src/models/post.rs` 中对应的公开结构体。
3. 修改 `build.rs` 的 `generate_content_rs()`，把新字段写入生成的 Rust 代码。
4. 更新 `docs/content.md` 的字段表。

`src/content.rs` 只是 `include!`，不需要改。

### 新增 artifact 根级文件

如果需要把某个 `public/` 文件放到 `dist/` 根（例如验证文件），把它加入 `src/main.rs` 的 `ROOT_LEVEL_FILES`。默认只有 `.nojekyll` 和 `CNAME`。

## 代码约束

### base 前缀

子路径部署依赖两层：

- `Router base=base_prefix()` 让路由和 `Link` 生成带前缀的 URL。
- `full_href()` 对站内绝对 href 加前缀。

因此：

- 不要在内链中硬编码 `/foo`，使用 `Link` 或 `full_href("/foo")`。
- Markdown 正文中的链接不会被改写；内容作者需要自行处理。
- 不要拼接 `base_prefix()` 后再手动加 `/` 造成双斜杠；`full_href` 已经处理。

### 路由可枚举

静态生成器只会写 `RouteList::generate(App)` 能收集到的路由。新增路由时检查：

- `SsrMode::Static(...)`；
- 路径参数有完整的 `prerender_params`；
- 结果路径以 base 开头（`main.rs` 会校验）。

### 编译期内容

`build.rs` 在编译期读取内容，生成的 HTML 会作为字符串常量嵌入。不要在页面组件中读取文件或访问网络；这些代码在生成阶段只执行一次，并且没有运行时环境。

### 可信内容

`MarkdownHtml` 使用 `inner_html` 注入预生成的 HTML，Markdown 中的原始 HTML 也会原样进入页面。当前假设内容源可信；不要把未经审核的第三方 Markdown 直接接入构建流程。

## 验证流程

仓库没有测试套件，修改后按以下步骤验证：

1. 构建并生成：

```bash
cargo run
```

2. 检查关键产物：

```bash
test -f dist/index.html
test -f dist/404.html
test -f dist/style.css
test -f dist/posts/rust/memory-layout/index.html
grep -o 'href="[^"]*"' dist/index.html | head
```

3. 检查页面内容：

```bash
grep -o '<h1>[^<]*</h1>' dist/about/index.html
grep -c 'code-block' dist/posts/rust/memory-layout/index.html
grep -o '<title>[^<]*</title>' dist/index.html
```

4. 验证子路径部署：

```bash
MAKI_BASE_URL=/maki cargo run
test -f dist/maki/index.html
test -f dist/maki/404.html
test -f dist/index.html
grep -o 'href="[^"]*"' dist/maki/index.html | head
```

确认 `dist/index.html` 是跳转页，且站内链接带 `/maki` 前缀。

5. 恢复默认构建：

```bash
cargo run
```

6. 如需视觉检查，启动本地服务器并实际打开页面：

```bash
caddy file-server --listen :8080 --browse --root dist
```

重点检查导航、文章页、标签页、归档树、深色模式首屏不闪烁、404 页面样式。

## 已知限制

| 限制 | 说明 |
| --- | --- |
| 无分页 | `Home` 直接遍历 `ALL_POSTS`，文章多时首页会很长 |
| 无 RSS / sitemap / 搜索 | 当前生成器未实现 |
| 页面标题固定 | 所有页面 `<title>` 都是 `Maki`，没有按文章/页面生成标题和 meta |
| 无客户端 hydration | 页面是纯静态 HTML，交互仅限内联主题脚本 |
| 只有根层 Markdown 是页面 | `scan_pages()` 不递归，内容目录子目录中的 Markdown 会被忽略、不生成页面；`/:slug` 也只匹配单段路径 |
| 中文自动摘要可能失败 | 未写 `summary` 时按 200 字节截断，可能 panic |
| 正文标题可能重复 | 模板和 Markdown 都可能输出 `<h1>` |
| Markdown 链接不处理 base | 子路径部署时站内绝对链接需自行处理 |
| 标签原样进 URL | 标签未做百分号编码，建议使用简短 ASCII 标识 |
| `dist/` 全量重建 | 每次生成都会删除整个 `dist/` |
| 无测试套件 | 目前依赖构建和产物检查 |

## 文档维护

修改以下行为时同步更新文档：

- `build.rs` 的 frontmatter 字段、排序、摘要、Markdown 选项；
- `src/app.rs` 的路由表；
- `src/main.rs` 的输出结构、CSS 顺序、根级文件、404/重定向逻辑；
- `MAKI_CONTENT_DIR`、`MAKI_BASE_URL` 的语义；
- 部署 workflow 的上传路径和 base 配置。
