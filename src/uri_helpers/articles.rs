use crate::models::Article;
use crate::uri_helpers::root_uri;

pub fn articles_uri() -> String {
    let mut uri = root_uri();
    if !uri.ends_with("/") {
        uri.push_str("/");
    }

    uri.push_str("articles");
    uri
}

pub fn article_uri(article: &Article) -> String {
    let mut uri = articles_uri();
    uri.push_str("/");
    uri.push_str(&article.id.to_string());

    uri
}

pub fn edit_article_uri(article: &Article) -> String {
    let mut uri = articles_uri();
    uri.push_str("/");
    uri.push_str(&article.id.to_string());
    uri.push_str("/edit");

    uri
}

pub fn new_article_uri() -> String {
    let mut uri = articles_uri();
    uri.push_str("/new");

    uri
}
