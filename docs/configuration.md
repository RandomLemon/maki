# 配置、构建产物与部署

## 环境变量

两个变量都在**构建期**读取，修改后必须重新构建；运行时不再读取。

| 变量 | 默认值 | 作用 | 失败行为 |
| --- | --- | --- | --- |
| `MAKI_CONTENT_DIR` | `<项目根>/content` | 指定内容目录，可指向独立目录或独立仓库 | 显式设置但目标不是目录时构建 panic |
| `MAKI_BASE_URL` | `/` | 站点部署的 URL 前缀，例如 `/maki` | 不以 `/` 开头时构建 panic |

### `MAKI_CONTENT_DIR`

```bash
# 相对路径按 Cargo.toml 所在目录解析
MAKI_CONTENT_DIR=./my-content cargo run

# 绝对路径
MAKI_CONTENT_DIR=/srv/maki-content cargo run
```

细节：

- 变量未设置或值为空字符串时，回退到 `<项目根>/content`。
- 相对路径不是相对当前 shell，而是相对 `CARGO_MANIFEST_DIR`。
- 显式设置时，目录必须存在且是目录，否则构建失败；默认 `content/` 缺失时按空内容处理。
- 目录内 `posts/` 可以不存在，此时文章集合为空；内容目录根部的 `.md` 文件是独立页面，子目录中的 Markdown 不参与页面扫描。
- 目录中的非 `.md` 文件（图片、PDF 等）会按相对路径复制到输出根；以 `.` 开头的项和符号链接不复制，`.md` 文件本身也不复制。
- 支持把内容放在独立仓库，在 CI 中先检出再通过该变量指向。

### `MAKI_BASE_URL`

`build.rs` 会做规范化：

1. 去掉末尾所有 `/`。
2. 结果为空时视为 `/`。
3. 结果必须以 `/` 开头，否则 panic。
4. 生成的 `BASE_URL` 常量不带末尾斜杠（根部署时为 `/`）。

`src/base.rs` 再把根部署映射为空字符串，方便直接拼接：

| 设置值 | 规范化后的 `BASE_URL` | `base_prefix()` | 输出目录 |
| --- | --- | --- | --- |
| 未设置 | `/` | `""` | `dist/` |
| `/` | `/` | `""` | `dist/` |
| 空字符串 | `/` | `""` | `dist/` |
| `/maki` | `/maki` | `/maki` | `dist/maki/` |
| `/maki/` | `/maki` | `/maki` | `dist/maki/` |
| `/foo/bar` | `/foo/bar` | `/foo/bar` | `dist/foo/bar/` |
| `maki` | 构建失败 | — | — |
| `https://example.com/maki` | 构建失败 | — | — |

不要把完整 URL 传给 `MAKI_BASE_URL`；它只接受绝对路径前缀。GitHub Pages 项目站点通常使用仓库名，例如 `/maki`。

站内链接通过 `full_href()` 或 `Link` 组件加前缀：

```rust
full_href("/archive") // base 为 /maki 时得到 /maki/archive
```

Markdown 正文中以单个 `/` 开头的站内绝对链接和图片会在 `build.rs` 中统一加 base 前缀；相对路径、外部 URL 和原始 HTML 中的链接不处理。已经带当前 base 的路径不会重复加前缀。

## 构建命令

```bash
# 开发构建并生成站点
cargo run

# release 构建并生成站点
cargo run --release

# 只构建二进制,不生成站点
cargo build --release
```

生成过程会打印每个输出文件：

```text
generated: dist/index.html
generated: dist/about/index.html
...
copied root-level file: .nojekyll
generated: dist/404.html
site generated in `dist`
```

## 输出目录

### 根部署（`MAKI_BASE_URL=/` 或未设置）

```text
dist/
├── 404.html
├── .nojekyll
├── style.css
├── index.html
├── about/index.html
├── archive/index.html
├── tags/index.html
├── tag/
│   ├── rust/index.html
│   └── ...
├── images/avatar.png       # 来自内容目录的资源
└── posts/
    ├── leptos/ssg-guide/index.html
    └── rust/memory-layout/index.html
```

### 子路径部署（例如 `MAKI_BASE_URL=/maki`）

```text
dist/
├── .nojekyll              # 根级文件,来自 public/
├── index.html             # 跳转页 → /maki/
└── maki/
    ├── 404.html
    ├── style.css
    ├── index.html
    ├── about/index.html
    ├── archive/index.html
    ├── tags/index.html
    ├── tag/...
    ├── images/avatar.png  # 来自内容目录的资源
    └── posts/...
```

说明：

- `content/` 中的非 `.md` 资源复制到站点根目录，保留相对路径（子路径部署时是 `dist/maki/`）。
- `public/` 中的普通资源复制到站点根目录（子路径部署时是 `dist/maki/`）。
- `.nojekyll`、`CNAME` 是例外，固定复制到 `dist/` 根目录。
- 子路径部署时，`dist/index.html` 是重定向页，不属于站点子目录。
- `404.html` 位于站点根目录内部（子路径部署时是 `dist/maki/404.html`）。

## 生成步骤

`src/main.rs` 的顺序：

1. 根据 `base_prefix()` 计算输出根目录。
2. 如果 `dist/` 存在，**整个删除**。
3. 创建输出根目录。
4. 把 `CONTENT_DIR` 中的非 `.md` 资源按相对路径复制到输出根目录。
5. 通过 `RouteList::generate(App)` + `into_static_paths()` 枚举静态路由。
6. 对每个路径调用 `App().to_html()`，写入 `<route>/index.html`。
7. 合并 `public/styles/*.css` 到 `<out_root>/style.css`。
8. 递归复制 `public/` 到输出根目录，排除 `styles`。
9. 把 `.nojekyll`、`CNAME` 复制到 `dist/` 根目录。
10. 子路径部署时写 `dist/index.html` 重定向页。
11. 渲染并写入 `<out_root>/404.html`。

注意：第 2 步删除的是整个 `dist/`，不是只删除当前 base 对应的子目录。不要在其他流程中把需要保留的文件放进 `dist/`。

## 样式合并

`merge_styles()` 按以下固定顺序拼接文件：

| 顺序 | 文件 | 主要职责 |
| --- | --- | --- |
| 1 | `variables.css` | 浅色/深色主题 CSS 变量、字体、间距 |
| 2 | `base.css` | reset、基础排版、链接、图片 |
| 3 | `layout.css` | `main`、`.page`、标题等布局 |
| 4 | `navbar.css` | 导航栏、主题切换按钮 |
| 5 | `post.css` | 文章列表、文章卡片、文章详情元信息 |
| 6 | `markdown.css` | `.markdown-body` 正文排版 |
| 7 | `syntax.css` | syntect 输出的代码高亮 class |
| 8 | `archive.css` | 归档目录树 |
| 9 | `tags.css` | 标签云 |
| 10 | `footer.css` | 页脚 |
| 11 | `not-found.css` | 404 页面 |

规则：

- 文件不存在时跳过，不报错。
- 只拼接，不压缩、不加工。
- 新增 CSS 文件必须同时加入 `src/main.rs` 的 `merge_styles()` 列表，否则不会进入 `style.css`。
- 不要直接在 HTML 中引用多个样式文件；`App` 只引用 `{base}/style.css`。

## 根级文件与 404

### `.nojekyll` 与 `CNAME`

`ROOT_LEVEL_FILES` 定义了需要放在 artifact 根目录的文件：

```rust
const ROOT_LEVEL_FILES: &[&str] = &[".nojekyll", "CNAME"];
```

它们从 `public/` 读取，复制到 `dist/` 根目录。用途：

- `.nojekyll`：让 GitHub Pages 跳过 Jekyll 处理。
- `CNAME`：自定义域名。

### 404

`404.html` 通过渲染一个不存在的 URL 触发 `NotFound` fallback，因此带有导航和样式。GitHub Pages 等支持自定义 404 的托管会自动使用它。

## 部署到 GitHub Pages

仓库自带 `.github/workflows/deploy.yml`：

- 触发：推送任意 tag，或发布 GitHub Release（`release: types: [published]`）。推送到 `main` 不会触发部署。
- 权限：`contents: read`、`pages: write`、`id-token: write`。
- 构建：
  - 安装 Nix 和缓存；
  - `MAKI_BASE_URL=/maki nix develop --command cargo run --release`；
  - 上传 `dist/maki` 作为 Pages artifact。
- 部署：`actions/deploy-pages@v4`。

这意味着站点默认部署在 `https://<owner>.github.io/maki/`。

### 适配自己的仓库

如果仓库不叫 `maki`，或者部署到用户/组织主页（根路径），需要同时调整：

1. `MAKI_BASE_URL`（例如改为 `/` 或你的仓库名）。
2. `upload-pages-artifact` 的 `path`：
   - 根部署通常上传 `dist`；
   - 子路径部署通常上传 `dist/<base>`，默认 workflow 是 `dist/maki`。

### 部署路径注意事项

当前生成器把以下文件放在 `dist/` 根，而不是 `dist/<base>/`：

- `dist/index.html`（子路径跳转页）
- `dist/.nojekyll`
- `dist/CNAME`

默认 workflow 只上传 `dist/maki`，因此这些根级文件**不会进入 artifact**：

- 跳转页不部署：访问域名根路径不会自动跳到 `/maki/`；GitHub 项目站点通常直接访问 `/maki/` 即可。
- `.nojekyll`、`CNAME` 不部署：如果依赖它们（例如自定义域名文件），需要把上传路径改为 `dist`，或在部署前把它们复制到 `dist/maki/`。

### 自定义域名

把 `CNAME` 放在 `public/CNAME`。如果继续只上传 `dist/maki`，需要确保该文件最终位于 artifact 根目录（即 `dist/maki/CNAME`），否则 GitHub Pages 不会读取到它。也可以直接在仓库的 Pages 设置中配置域名。

## 本地预览

```bash
# 根部署
cargo run
caddy file-server --listen :8080 --browse --root dist

# 子路径部署
MAKI_BASE_URL=/maki cargo run
caddy file-server --listen :8080 --browse --root dist
# 打开 http://localhost:8080/maki/
```

任意静态文件服务器都可以；关键是服务器要能对目录返回 `index.html`。本站不需要 SPA fallback，所有页面都是真实存在的 `index.html`。

## 重建触发

以下变化会让 Cargo 重新执行 `build.rs`：

- 内容目录中的文件变化；
- `MAKI_CONTENT_DIR` 变化；
- `MAKI_BASE_URL` 变化；
- `build.rs` 本身或相关源码变化。

每次运行生成器都会全量重建 `dist/`。如果怀疑缓存导致内容未更新，可执行：

```bash
cargo clean -p maki
cargo run
```

## 故障排查

| 现象 | 原因 | 处理 |
| --- | --- | --- |
| `MAKI_BASE_URL must be an absolute path starting with '/'` | 传了 `maki`、完整 URL 等 | 改为 `/maki` 这类绝对路径 |
| `MAKI_CONTENT_DIR is not a directory` | 路径不存在或不是目录 | 检查路径；相对路径按项目根解析 |
| `failed to parse frontmatter` | 文章 YAML 格式错误或缺少 `title` | 修正 frontmatter；文章 `title` 必填 |
| `assertion failed: self.is_char_boundary(new_len)` | 未写 `summary`，自动摘要按 200 字节截断时切在多字节字符中间 | 为文章显式填写 `summary` |
| `route path ... not prefixed by base ...` | 路由生成结果与 base 不一致，通常是自定义路由/参数逻辑有误 | 检查路由是否由 `App` 的 `Router base` 生成 |
| 子路径部署后裸 HTML 中的链接 404 | 原始 HTML 里的 URL 不会被改写 | 手动写成 `/maki/foo`，或改用 Markdown 语法 |
| 图片或附件 404 | 资源不在内容目录/`public/` 中，或 Markdown 路径写错 | 确认 `dist/` 中存在对应文件；内容目录中的非 `.md` 文件按相对路径复制，隐藏项和 `.md` 不复制 |
| 页面没有样式 | `style.css` 未生成或 base 配置不一致 | 检查 `public/styles`、`merge_styles()` 列表和 `MAKI_BASE_URL` |
| 修改内容后页面没变 | 未重新运行生成器，或内容目录不是实际使用的目录 | 重新 `cargo run`，确认 `MAKI_CONTENT_DIR` |
