use leptos::prelude::*;
use leptos_router::{RouteList, location::RequestUrl};
use maki::App;
use maki::content::BASE_URL;
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
    let base = BASE_URL.trim_end_matches('/');
    if base.is_empty() {
        PathBuf::from("dist")
    } else {
        PathBuf::from("dist").join(base.trim_start_matches('/'))
    }
}

fn main() {
    let out_root = output_root();

    if Path::new("dist").exists() {
        fs::remove_dir_all("dist").expect("failed to clean dist directory");
    }
    fs::create_dir_all(&out_root).expect("failed to create output directory");

    let routes = Owner::new()
        .with(|| {
            provide_context(RequestUrl::new("/"));
            RouteList::generate(App)
        })
        .unwrap_or_default();

    let raw_paths = futures::executor::block_on(routes.into_static_paths());
    let unique_paths: HashSet<String> = raw_paths.iter().map(|p| clean_path(p.as_ref())).collect();
    let mut sorted_paths: Vec<String> = unique_paths.into_iter().collect();
    sorted_paths.sort();

    let base = BASE_URL.trim_end_matches('/');
    for path in sorted_paths {
        let html = render_path(&path);
        let relative = path
            .strip_prefix(base)
            .unwrap_or(&path)
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
        copy_dir_all("public", &out_root, &["styles"]).expect("failed to copy public assets");
    }

    println!("site generated in `{}`", out_root.display());
}
