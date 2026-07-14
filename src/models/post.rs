use chrono::NaiveDate;

#[derive(Clone, Debug)]
pub struct Post {
    pub slug: &'static str,
    pub full_path: &'static str,
    pub title: &'static str,
    pub date: NaiveDate,
    pub tags: &'static [&'static str],
    pub summary: &'static str,
    pub content_html: &'static str,
}

#[derive(Clone, Debug)]
pub struct StaticPage {
    pub slug: &'static str,
    pub title: &'static str,
    pub content_html: &'static str,
}

#[derive(Clone, Debug)]
pub struct ContentNode {
    pub name: &'static str,
    pub path: &'static str,
    pub is_post: bool,
    pub children: &'static [ContentNode],
}
