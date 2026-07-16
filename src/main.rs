use leptos::prelude::*;
use leptos_router::{RouteList, location::RequestUrl};
use maki::App;
use maki::base::base_prefix;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

fn render_path(path: &str) -> String {
    let owner = Owner::new();
    owner.with(|| {
        provide_context(RequestUrl::new(path));
        App().to_html()
    })
}

fn copy_dir_all(
    src: impl AsRef<Path>,
    dst: impl AsRef<Path>,
    exclude: &[&str],
) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_string();
        if exclude.contains(&name.as_str()) {
            continue;
        }
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(&name), exclude)?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(&name))?;
        }
    }
    Ok(())
}

fn clean_path(path: &str) -> String {
    let mut result = path.to_string();
    while result.contains("//") {
        result = result.replace("//", "/");
    }
    result
}

fn merge_styles(out: &Path) -> std::io::Result<()> {
    let order = [
        "variables.css",
        "base.css",
        "layout.css",
        "navbar.css",
        "post.css",
        "markdown.css",
        "archive.css",
        "tags.css",
        "footer.css",
        "not-found.css",
    ];
    let mut merged = String::new();
    for name in order {
        let path = Path::new("public/styles").join(name);
        if path.exists() {
            merged.push_str(&fs::read_to_string(path)?);
            merged.push('\n');
        }
    }
    fs::write(out, merged)?;
    Ok(())
}

fn output_root() -> PathBuf {
    let base = base_prefix();
    if base.is_empty() {
        PathBuf::from("dist")
    } else {
        PathBuf::from("dist").join(base.trim_start_matches('/'))
    }
}

/// 必须位于站点 artifact 根目录(`dist/`)而非子目录的文件,
/// 例如 `.nojekyll`(禁用 GitHub Pages 的 Jekyll 处理)、`CNAME`(自定义域名)。
const ROOT_LEVEL_FILES: &[&str] = &[".nojekyll", "CNAME"];

fn copy_root_level_files(dst: &Path) {
    for name in ROOT_LEVEL_FILES {
        let src = Path::new("public").join(name);
        if src.exists() {
            fs::copy(&src, dst.join(name)).expect("failed to copy root-level file");
            println!("copied root-level file: {name}");
        }
    }
}

fn write_redirect_index(base: &str) {
    let target = if base.is_empty() {
        "/".to_string()
    } else {
        format!("{}/", base)
    };
    let html = format!(
        "<!DOCTYPE html>\n\
         <html lang=\"zh-CN\">\n\
         <head>\n\
         <meta charset=\"utf-8\" />\n\
         <title>Maki</title>\n\
         <link rel=\"canonical\" href=\"{target}\" />\n\
         <meta http-equiv=\"refresh\" content=\"0; url={target}\" />\n\
         </head>\n\
         <body>\n\
         <script>location.replace({target:?})</script>\n\
         <p>Redirecting to <a href=\"{target}\">{target}</a>…</p>\n\
         </body>\n\
         </html>\n"
    );
    fs::write(Path::new("dist").join("index.html"), html)
        .expect("failed to write root redirect index.html");
    println!("generated: dist/index.html (redirect -> {target})");
}

fn write_not_found(out_root: &Path) {
    // NotFound 路由声明为 path!("/*") 的 SSG 静态路由,但 leptos_router 不会枚举
    // 具体的 404 路径。这里用一个不匹配任何具体页面的 URL 来触发 fallback,
    // 从而渲染带样式的 NotFound 组件并写入 `404.html`。
    let base = base_prefix();
    let bogus_url = format!("{}/__nonexistent__", base);
    let html = render_path(&bogus_url);
    fs::write(out_root.join("404.html"), html).expect("failed to write 404.html");
    println!("generated: {}", out_root.join("404.html").display());
}

fn main() {
    let out_root = output_root();
    let base = base_prefix();

    if Path::new("dist").exists() {
        fs::remove_dir_all("dist").expect("failed to clean dist directory");
    }
    fs::create_dir_all(&out_root).expect("failed to create output directory");

    let routes = Owner::new()
        .with(|| {
            provide_context(RequestUrl::new(base));
            RouteList::generate(App)
        })
        .unwrap_or_default();

    let raw_paths = futures::executor::block_on(routes.into_static_paths());
    let unique_paths: HashSet<String> = raw_paths.iter().map(|p| clean_path(p.as_ref())).collect();
    let mut sorted_paths: Vec<String> = unique_paths.into_iter().collect();
    sorted_paths.sort();

    for path in sorted_paths {
        let html = render_path(&path);
        let relative = path
            .strip_prefix(base)
            .unwrap_or_else(|| panic!("route path {path:?} not prefixed by base {base:?}"))
            .trim_start_matches('/')
            .trim_end_matches('/');
        let file = if relative.is_empty() {
            out_root.join("index.html")
        } else {
            out_root.join(relative).join("index.html")
        };
        fs::create_dir_all(file.parent().unwrap()).expect("failed to create output directory");
        fs::write(&file, html).expect("failed to write html file");
        println!("generated: {}", file.display());
    }

    merge_styles(&out_root.join("style.css")).expect("failed to merge styles");

    if Path::new("public").exists() {
        // 公共资源拷入站点目录;根级文件(如 .nojekyll)单独拷到 dist/ 根。
        let mut exclude: Vec<&str> = vec!["styles"];
        exclude.extend_from_slice(ROOT_LEVEL_FILES);
        copy_dir_all("public", &out_root, &exclude).expect("failed to copy public assets");
        copy_root_level_files(Path::new("dist"));
    }

    // 子路径部署时,在 artifact 根放一个跳转页,把访问 `/` 的访客引导到 `{base}/`。
    if !base.is_empty() {
        write_redirect_index(base);
    }

    // 渲染带样式的 404 页(GitHub Pages 等会自动回退到 404.html)。
    write_not_found(&out_root);

    println!("site generated in `{}`", out_root.display());
}