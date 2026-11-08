# 内容编写指南

## 目录结构

默认内容目录是仓库根目录的 `content/`：

```text
content/
├── posts/
│   ├── rust/
│   │   └── memory-layout.md      → /posts/rust/memory-layout
│   └── leptos/
│       └── ssg-guide.md          → /posts/leptos/ssg-guide
└── pages/
    └── about.md                  → /about
```

也可以通过 `MAKI_CONTENT_DIR` 指向任意目录或独立内容仓库：

```bash
MAKI_CONTENT_DIR=/path/to/maki-content cargo run
```

相对路径按 `Cargo.toml` 所在目录解析。完整规则见 [configuration.md](./configuration.md)。

## 文章

文章放在 `posts/` 下，使用 Markdown + YAML frontmatter。

### frontmatter

```markdown
---
title: Rust 内存布局浅析
date: 2026-07-15
tags: [rust, memory]
summary: 介绍 Rust 中常见数据结构的内存布局，帮助理解所有权与借用。
---

# Rust 内存布局浅析

正文...
```

| 字段 | 类型 | 必填 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| `title` | 字符串 | 是 | 无 | 文章标题；缺失或 frontmatter 解析失败会导致构建 panic |
| `date` | 字符串 | 否 | 文件修改时间的 UTC 日期 | 按 `NaiveDate` 解析，推荐 `YYYY-MM-DD`；解析失败静默回退到 `1970-01-01` |
| `tags` | 字符串列表 | 否 | 空列表 | 标签区分大小写；每个标签会生成一个 `/tag/<tag>` 页面 |
| `summary` | 字符串 | 否 | 自动摘要 | 首页和标签页展示的摘要；建议显式填写，见下文“自动摘要” |

未知字段会被忽略，不会报错。

### slug 与 URL

- slug 是文件相对 `posts/` 的路径去掉 `.md`，例如 `rust/memory-layout.md` → `rust/memory-layout`。
- 文章 URL 固定为 `/posts/{slug}`。
- 子目录可以无限嵌套，文章 URL 会包含同样的层级。
- 归档树中的目录来自 slug 的路径段；目录名就是路径段本身，文章节点显示 `title`。
- 建议使用 ASCII、短横线分隔的文件名；slug 会原样进入 URL，未做百分号编码。

### 排序

`ALL_POSTS` 按 `date` 降序排列。日期相同的文章之间没有显式排序，顺序取决于文件扫描顺序，因此不要依赖同日期文章的先后。

## 独立页面

页面放在 `pages/` 下，通常用于“关于”等单页。

### 带 frontmatter

```markdown
---
title: 关于
---

这里是正文。
```

| 字段 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- |
| `title` | 字符串 | 是 | 页面标题 |

页面 slug 是文件相对 `pages/` 的路径去掉 `.md`，URL 为 `/{slug}`。

### 不带 frontmatter

```markdown
# 关于

这里是正文。
```

当 frontmatter 解析失败时：

1. 取正文第一行以 `# ` 开头的内容作为标题；
2. 没有这样的行时，标题回退为 `"关于"`；
3. 整个文件（包括第一行）都作为 Markdown 正文渲染。

### 页面必须保持单层

`pages/` 的扫描是递归的，但站点只有 `/:slug` 这一条页面路由，它**只匹配单段路径**。因此：

```text
pages/about.md          → /about          ✅ 正常
pages/nested/page.md    → /nested/page    ❌ 会渲染成 404 内容
```

嵌套页面虽然会被生成目录和 HTML 文件，但路由匹配不到 `StaticPageView`，实际输出的是 `NotFound`。请把页面直接放在 `pages/` 根目录。

## Markdown 规则

Maki 使用 `pulldown-cmark` 的默认 CommonMark 选项：

- 支持标题、段落、列表、引用、链接、图片、加粗/斜体、行内代码、围栏代码块等 CommonMark 语法。
- **未启用 GFM 扩展**：表格、任务列表、删除线、脚注等不会按扩展语法渲染。表格语法会作为普通段落文本输出。
- 原始 HTML 会原样进入生成的 HTML。
- 正文中的链接和图片按原样输出，不会自动加 `base` 前缀。子路径部署时，站内绝对链接需要在 Markdown 中自行处理，或改用相对链接。

### 代码块

围栏代码块的语言标记用于 `syntect` 语法查找：

````markdown
```rust
fn main() {}
```
````

- 能识别的语言会生成带 class 的高亮 `<span>`，配色由 `public/styles/syntax.css` 控制。
- 未知语言回退到纯文本语法。
- 没有语言标记或使用缩进代码块时，输出经过 HTML 转义的纯文本代码块。
- 输出结构固定为 `<pre class="code-block"><code>…</code></pre>`。

### 标题重复

文章模板和页面模板都会在正文外渲染一个 `<h1>{title}</h1>`。如果 Markdown 正文开头也写 `# 标题`，最终页面会出现两个一级标题。示例内容目前就是这种形式；如需避免重复，可去掉正文中的一级标题，或调整页面模板。

## 归档树

`/archive` 使用 `CONTENT_TREE` 展示文章目录：

- 目录节点按路径段生成，只显示文件夹名，不可点击。
- 文章节点显示 `title`，链接到 `/posts/{slug}`。
- 同一层中目录排在文章前面，然后分别按名称字典序排序。
- 没有文章的目录不会凭空出现，因为目录节点只由文章 slug 推导。

## 自动摘要

`summary` 缺失时，构建脚本会：

1. 取渲染后的文章 HTML；
2. 用简单的标签剥离逻辑得到纯文本；
3. 调用 `String::truncate(200)`，保留前 **200 字节**。

这有两个实际影响：

- 摘要可能在中文字符中间截断，`String::truncate` 会 panic，导致构建失败。**中文等非 ASCII 内容建议始终显式填写 `summary`。**
- 摘要长度按字节而不是字符计算。

## 内容更新与重建

内容在编译期被读取并嵌入生成的 Rust 代码。修改 Markdown 后需要重新运行：

```bash
cargo run
```

`build.rs` 已声明监听内容目录，Cargo 会在内容变化时重新执行构建脚本。切换 `MAKI_CONTENT_DIR` 或 `MAKI_BASE_URL` 也会触发重建。

## 写作检查清单

- [ ] 文章 frontmatter 有 `title`，日期使用 `YYYY-MM-DD`。
- [ ] 显式提供 `summary`，尤其是中文内容。
- [ ] 标签使用简短、稳定、区分大小写的标识，避免 `/`、`?`、`#` 和空格。
- [ ] 页面直接放在 `pages/` 根目录，不嵌套子目录。
- [ ] 正文如需站内绝对链接，确认子路径部署下的 `base` 前缀。
- [ ] 运行 `cargo run` 后检查 `dist/` 中对应路径的 HTML。
