use chrono::{Datelike, NaiveDate};
use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag, TagEnd};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;
use walkdir::WalkDir;
use yaml_front_matter::YamlFrontMatter;

#[derive(Debug, Deserialize)]
struct PostMeta {
    title: String,
    #[serde(default)]
    date: Option<String>,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    summary: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PageMeta {
    title: String,
}

#[derive(Debug, Clone)]
struct Post {
    slug: String,
    full_path: String,
    title: String,
    date: NaiveDate,
    tags: Vec<String>,
    summary: String,
    content_html: String,
}

#[derive(Debug, Clone)]
struct StaticPage {
    slug: String,
    title: String,
    content_html: String,
}

#[derive(Debug, Clone)]
struct ContentNode {
    name: String,
    path: String,
    is_post: bool,
    children: Vec<ContentNode>,
}

fn escape_rust_string(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '\\' => "\\\\".to_string(),
            '"' => "\\\"".to_string(),
            '\n' => "\\n".to_string(),
            '\r' => "\\r".to_string(),
            '\t' => "\\t".to_string(),
            c => c.to_string(),
        })
        .collect()
}

fn parse_date(s: &str) -> NaiveDate {
    s.parse::<NaiveDate>()
        .unwrap_or_else(|_| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap())
}

fn plain_text_summary(html: &str) -> String {
    let mut text = String::new();
    let mut in_tag = false;
    for c in html.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => text.push(c),
            _ => {}
        }
    }
    text.truncate(200);
    text
}

fn markdown_to_html(input: &str) -> String {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["base16-ocean.dark"];

    let parser = Parser::new(input);
    let mut events: Vec<Event> = Vec::new();
    let mut in_code = false;
    let mut lang = String::new();
    let mut code_text = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                in_code = true;
                lang = match kind {
                    CodeBlockKind::Fenced(l) => l.to_string(),
                    _ => String::new(),
                };
            }
            Event::End(TagEnd::CodeBlock) => {
                in_code = false;
                let html = if lang.is_empty() {
                    format!("<pre><code>{}</code></pre>", html_escape(&code_text))
                } else {
                    let syntax = ss
                        .find_syntax_by_token(&lang)
                        .unwrap_or_else(|| ss.find_syntax_plain_text());
                    highlighted_html_for_string(&code_text, &ss, syntax, theme).unwrap_or_else(
                        |_| format!("<pre><code>{}</code></pre>", html_escape(&code_text)),
                    )
                };
                events.push(Event::Html(html.into()));
                code_text.clear();
                lang.clear();
            }
            Event::Text(text) if in_code => {
                code_text.push_str(&text);
            }
            other => events.push(other),
        }
    }

    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, events.into_iter());
    html
}

fn html_escape(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '&' => "&amp;".to_string(),
            '<' => "&lt;".to_string(),
            '>' => "&gt;".to_string(),
            '"' => "&quot;".to_string(),
            c => c.to_string(),
        })
        .collect()
}

fn scan_posts(dir: &Path) -> Vec<Post> {
    let mut posts = Vec::new();
    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("md"))
    {
        let path = entry.path();
        let rel = path.strip_prefix(dir).unwrap();
        let slug = rel.with_extension("").to_string_lossy().replace('\\', "/");
        let full_path = format!("/posts/{}", slug);

        let raw = fs::read_to_string(path).expect("failed to read markdown file");
        let doc: yaml_front_matter::Document<PostMeta> =
            YamlFrontMatter::parse::<PostMeta>(&raw).expect("failed to parse frontmatter");

        let content_html = markdown_to_html(&doc.content);
        let summary = doc
            .metadata
            .summary
            .unwrap_or_else(|| plain_text_summary(&content_html));

        posts.push(Post {
            slug,
            full_path,
            title: doc.metadata.title,
            date: doc
                .metadata
                .date
                .map(|d| parse_date(&d))
                .unwrap_or_else(|| {
                    fs::metadata(path)
                        .ok()
                        .and_then(|m| m.modified().ok())
                        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                        .map(|d| {
                            chrono::DateTime::from_timestamp(d.as_secs() as i64, 0)
                                .unwrap()
                                .date_naive()
                        })
                        .unwrap_or_else(|| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap())
                }),
            tags: doc.metadata.tags,
            summary,
            content_html,
        });
    }
    posts.sort_by(|a, b| b.date.cmp(&a.date));
    posts
}

fn scan_pages(dir: &Path) -> Vec<StaticPage> {
    let mut pages = Vec::new();
    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("md"))
    {
        let path = entry.path();
        let rel = path.strip_prefix(dir).unwrap();
        let slug = rel.with_extension("").to_string_lossy().replace('\\', "/");

        let raw = fs::read_to_string(path).expect("failed to read markdown file");
        let doc: yaml_front_matter::Document<PageMeta> =
            YamlFrontMatter::parse::<PageMeta>(&raw).expect("failed to parse frontmatter");

        pages.push(StaticPage {
            slug,
            title: doc.metadata.title,
            content_html: markdown_to_html(&doc.content),
        });
    }
    pages
}

fn build_content_tree(posts: &[Post]) -> ContentNode {
    let mut root = ContentNode {
        name: "posts".to_string(),
        path: "/archive".to_string(),
        is_post: false,
        children: Vec::new(),
    };

    for post in posts {
        let parts: Vec<&str> = post.slug.split('/').collect();
        let mut current = &mut root;

        for (i, part) in parts.iter().enumerate() {
            if i == parts.len() - 1 {
                current.children.push(ContentNode {
                    name: post.title.clone(),
                    path: post.full_path.clone(),
                    is_post: true,
                    children: Vec::new(),
                });
            } else {
                let idx = current
                    .children
                    .iter()
                    .position(|c| !c.is_post && c.name == *part);
                current = match idx {
                    Some(i) => &mut current.children[i],
                    None => {
                        let path = format!(
                            "{}/{}",
                            if current.path == "/archive" {
                                "/archive".to_string()
                            } else {
                                current.path.clone()
                            },
                            part
                        );
                        current.children.push(ContentNode {
                            name: part.to_string(),
                            path,
                            is_post: false,
                            children: Vec::new(),
                        });
                        let last = current.children.len() - 1;
                        &mut current.children[last]
                    }
                };
            }
        }
    }

    sort_tree(&mut root);
    root
}

fn sort_tree(node: &mut ContentNode) {
    node.children.sort_by(|a, b| match (a.is_post, b.is_post) {
        (false, true) => std::cmp::Ordering::Less,
        (true, false) => std::cmp::Ordering::Greater,
        _ => a.name.cmp(&b.name),
    });
    for child in &mut node.children {
        if !child.is_post {
            sort_tree(child);
        }
    }
}

fn generate_content_rs(
    posts: &[Post],
    pages: &[StaticPage],
    tree: &ContentNode,
    base_url: &str,
    out: &Path,
) {
    let mut buf = String::new();
    buf.push_str("use chrono::NaiveDate;\n\n");

    buf.push_str(&format!(
        "pub const BASE_URL: &str = \"{}\";\n\n",
        escape_rust_string(base_url)
    ));

    buf.push_str("pub static ALL_POSTS: &[Post] = &[\n");
    for post in posts {
        let date = post.date;
        let tags = post
            .tags
            .iter()
            .map(|t| format!("\"{}\"", escape_rust_string(t)))
            .collect::<Vec<_>>()
            .join(", ");
        buf.push_str(&format!(
            "    Post {{\n        slug: \"{}\",\n        full_path: \"{}\",\n        title: \"{}\",\n        date: NaiveDate::from_ymd_opt({}, {}, {}).unwrap(),\n        tags: &[{}],\n        summary: \"{}\",\n        content_html: \"{}\",\n    }},\n",
            escape_rust_string(&post.slug),
            escape_rust_string(&post.full_path),
            escape_rust_string(&post.title),
            date.year(),
            date.month(),
            date.day(),
            tags,
            escape_rust_string(&post.summary),
            escape_rust_string(&post.content_html),
        ));
    }
    buf.push_str("];\n\n");

    buf.push_str("pub static ALL_PAGES: &[StaticPage] = &[\n");
    for page in pages {
        buf.push_str(&format!(
            "    StaticPage {{\n        slug: \"{}\",\n        title: \"{}\",\n        content_html: \"{}\",\n    }},\n",
            escape_rust_string(&page.slug),
            escape_rust_string(&page.title),
            escape_rust_string(&page.content_html),
        ));
    }
    buf.push_str("];\n\n");

    buf.push_str("pub static CONTENT_TREE: &ContentNode = ");
    write_node(&mut buf, tree, true);
    buf.push_str(";\n");

    fs::write(out, buf).expect("failed to write generated content.rs");
}

fn write_node(buf: &mut String, node: &ContentNode, is_root: bool) {
    if is_root {
        buf.push_str("&");
    }
    buf.push_str("ContentNode {\n");
    buf.push_str(&format!(
        "        name: \"{}\",\n",
        escape_rust_string(&node.name)
    ));
    buf.push_str(&format!(
        "        path: \"{}\",\n",
        escape_rust_string(&node.path)
    ));
    buf.push_str(&format!("        is_post: {},\n", node.is_post));
    buf.push_str("        children: &[\n");
    for child in &node.children {
        write_node(buf, child, false);
        buf.push_str(",\n");
    }
    buf.push_str("        ],\n    }");
}

fn main() {
    println!("cargo::rerun-if-changed=content");
    println!("cargo::rerun-if-env-changed=MAKI_BASE_URL");

    let posts_dir = PathBuf::from("content/posts");
    let pages_dir = PathBuf::from("content/pages");

    let posts = if posts_dir.exists() {
        scan_posts(&posts_dir)
    } else {
        Vec::new()
    };

    let pages = if pages_dir.exists() {
        scan_pages(&pages_dir)
    } else {
        Vec::new()
    };

    let tree = build_content_tree(&posts);

    let base_url = std::env::var("MAKI_BASE_URL").unwrap_or_else(|_| "/".to_string());
    let base_url = base_url.trim_end_matches('/').to_string();
    let base_url = if base_url.is_empty() {
        "/".to_string()
    } else {
        base_url
    };

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out = PathBuf::from(out_dir).join("content.rs");
    generate_content_rs(&posts, &pages, &tree, &base_url, &out);
}
