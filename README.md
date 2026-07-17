# maki

`maki` 是一个利用markdown生成静态个人博客网站的项目。本项目致力于易用性和扩展性，只需编写markdown文件，编译后自动生成静态网站，可部署至`GitHub Pages`等服务。利用`GitHub Actions`等工作流，还可以实现自动部署与更新。

## usage

1. 编写您的博客
在 `content` 文件夹中包含了您博客的内容。其中：
    - `posts` 中包含博客的内容。在markdown文件中使用yaml编写frontmatter，即可标记文章的标题、发布日期、标签和概要。
    - `pages` 可以按需编写一些其它页面，比如“关于”页面。

2. 编译您的网站
  - 手动编译：运行 `cargo run`，编译的结果在 `dist` 文件夹下。
  - 本地预览：运行一个http服务器即可预览：

```bash
caddy file-server --listen :8080 --browse --root dist
```

## about

Maki 是一个使用 Rust + Leptos 构建的静态个人博客框架。项目源码: [https://github.com/RandomLemon/maki](https://github.com/RandomLemon/maki)

## plan

- [x] markdown转换为网页
- [x] 文件目录转换为网页目录
- [x] 标签分类
- [ ] 风格化
- [x] 深色模式切换
- [ ] 其他功能...
