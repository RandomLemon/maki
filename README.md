# maki

`maki` 是一个利用markdown生成静态个人博客网站的项目。本项目致力于易用性和扩展性，只需编写markdown文件，编译后自动生成静态网站，可部署至`GitHub Pages`等服务。利用`GitHub Actions`等工作流，还可以实现自动部署与更新。

## usage

### 1. 编写内容

内容默认从项目根目录的 `content/` 读取，也可以放在任意独立目录或独立仓库，并通过 `MAKI_CONTENT_DIR` 指定：

```bash
MAKI_CONTENT_DIR=/path/to/maki-content cargo run
```

- 相对路径按 `Cargo.toml` 所在目录解析。
- 未设置时使用 `content/`；显式设置但目标不是目录时构建会失败。
- 内容目录结构：

```text
maki-content/
├── about.md
└── posts/
    └── ...
```

- `posts` 中包含博客文章。在 Markdown 文件中使用 YAML frontmatter 标记标题、发布日期、标签和概要。
- 内容目录根部的 `.md` 文件是独立页面，比如 `about.md` 会生成 `/about`；除 `posts/` 外的子目录中的 Markdown 不会被扫描为页面。

内容在编译期被读取并嵌入站点。`build.rs` 会监听内容目录，修改后重新运行 `cargo run` 即可。

### 2. 编译网站

- 手动编译：运行 `cargo run`，编译结果在 `dist/` 目录下。
- 本地预览：运行一个 HTTP 服务器即可预览：

```bash
caddy file-server --listen :8080 --browse --root dist
```

### 3. 内容与站点分仓（可选）

在 GitHub Actions 中先检出内容仓库，再把 `MAKI_CONTENT_DIR` 指向检出的目录：

```yaml
- name: Checkout content
  uses: actions/checkout@v4
  with:
    repository: owner/maki-content
    path: .content
    token: ${{ secrets.CONTENT_TOKEN }} # 公开仓库可省略

- name: Build site
  env:
    MAKI_BASE_URL: "/maki"
    MAKI_CONTENT_DIR: ${{ github.workspace }}/.content
  run: nix develop --command cargo run --release
```

如果希望内容仓库推送后自动重建站点，可在站点工作流的 `on` 中加入 `repository_dispatch`，由内容仓库的工作流调用 GitHub API 触发；也可以使用定时构建。

## about

Maki 是一个使用 Rust + Leptos 构建的静态个人博客框架。项目源码: [https://github.com/RandomLemon/maki](https://github.com/RandomLemon/maki)

## plan

- [x] markdown转换为网页
- [x] 文件目录转换为网页目录
- [x] 标签分类
- [ ] 风格化
- [x] 深色模式切换
- [ ] 其他功能...
