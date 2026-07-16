use crate::content::BASE_URL;

/// 返回规范化后的 base 前缀。
///
/// - 永远以 `/` 开头或为空字符串;
/// - 永远不以斜杠结尾;
/// - 默认根部署(`BASE_URL == "/"`)时返回 `""`,便于直接拼接路径而不产生双斜杠。
///
/// 依赖 `build.rs` 在编译期保证 `BASE_URL` 必须是绝对路径(以 `/` 开头)。
pub fn base_prefix() -> &'static str {
    match BASE_URL {
        "/" => "",
        other => other.trim_end_matches('/'),
    }
}

/// 把以 `/` 开头的站内 href 拼上 base 前缀,外部链接和相对链接原样返回。
pub fn full_href(href: &str) -> String {
    if href.starts_with('/') {
        format!("{}{}", base_prefix(), href)
    } else {
        href.to_string()
    }
}