# Maki 项目文档

Maki 是一个用 Rust + Leptos 0.7 构建的静态博客生成器。它在**编译期**扫描 Markdown、解析 YAML frontmatter、生成 Rust 常量，再由 Leptos 的 SSR 能力把每个路由渲染成纯静态 HTML。运行时不依赖 WASM、hydration 或后端服务，只有一段用于主题切换的内联 JavaScript。

## 快速开始

```bash
# 推荐:进入 Nix 开发环境(仓库自带 flake,也可在项目根执行 direnv allow)
nix develop

# 生成站点,输出到 dist/
cargo run

# 本地预览
caddy file-server --listen :8080 --browse --root dist
```

默认访问 `http://localhost:8080/`。部署到子路径时：

```bash
MAKI_BASE_URL=/maki cargo run
caddy file-server --listen :8080 --browse --root dist
# 访问 http://localhost:8080/maki/
```

## 文档地图

| 文档 | 内容 |
| --- | --- |
| [architecture.md](./architecture.md) | 构建流程、数据模型、路由表、渲染与主题机制 |
| [content.md](./content.md) | 内容目录、frontmatter 字段、Markdown 规则、归档树与写作注意事项 |
| [configuration.md](./configuration.md) | 环境变量、`base` 路径、输出目录、CSS 合并、部署与故障排查 |
| [development.md](./development.md) | 开发环境、扩展方式、代码约束、验证流程与已知限制 |

## 项目速览

| 项目 | 说明 |
| --- | --- |
| 输入 | `content/posts/**/*.md` 与 `content/*.md`，以及随站点发布的非 Markdown 资源；可由 `MAKI_CONTENT_DIR` 指向外部目录 |
| 输出 | `dist/` 下的静态 HTML、合并后的 `style.css`、`404.html`，以及 `content/`、`public/` 中的静态资源 |
| 技术栈 | Rust 2024 edition、Leptos 0.7.8(SSR)、leptos_router 0.7.8、pulldown-cmark、syntect、chrono |
| 内容处理 | Markdown 在编译期由 `build.rs` 处理并生成 `OUT_DIR/content.rs`；非 Markdown 资源在生成期按相对路径复制 |
| 运行模型 | 纯静态多页站点；无客户端路由，无 WASM hydration |
| 主题 | `localStorage` + `prefers-color-scheme`，页面加载前同步应用，支持 View Transitions 切换动画 |
| 部署 | GitHub Actions + Nix 构建，产物上传 GitHub Pages；也可部署到任意静态托管 |

## 源码目录

```text
.
├── build.rs                  # 编译期内容处理与代码生成
├── src/
│   ├── main.rs               # 静态站点生成器入口
│   ├── app.rs                # 根组件、路由定义、主题初始化脚本
│   ├── base.rs               # base 前缀与站内链接处理
│   ├── content.rs            # include!(OUT_DIR/content.rs)
│   ├── models/               # Post / StaticPage / ContentNode
│   ├── components/           # Navbar、Footer、Link、MarkdownHtml、ContentTree
│   └── pages/                # Home、Archive、Tags、TagPosts、PostDetail、StaticPageView、NotFound
├── public/
│   ├── styles/               # 按固定顺序合并为 dist/style.css
│   └── .nojekyll             # 复制到 dist/ 根目录
├── content/
│   ├── about.md              # 示例独立页面
│   └── posts/                # 示例文章
├── flake.nix                 # Nix 开发环境(含 caddy)
├── rust-toolchain.toml       # stable 工具链
└── .github/workflows/deploy.yml
```

## 事实来源

本文档依据仓库当前源码整理，关键行为以以下文件为准：

- `build.rs`：内容扫描、frontmatter 解析、Markdown 渲染、`content.rs` 生成。
- `src/main.rs`：路由枚举、HTML 输出、CSS 合并、静态资源复制、404 与重定向生成。
- `src/app.rs`：路由表、`base` 接入、主题初始化脚本。
- `src/base.rs`、`src/components/`、`src/pages/`：运行时渲染行为。

修改上述文件后应同步更新对应文档。
