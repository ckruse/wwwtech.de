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

pub fn articles_atom_uri() -> String {
    let mut uri = articles_uri();
    uri.push_str(".atom");
    uri
}

pub fn article_uri(article: &Article) -> String {
    let mut uri = articles_uri();
    uri.push_str("/");
    uri.push_str(&article.slug);

    uri
}

pub fn edit_article_uri(article: &Article) -> String {
    let mut uri = articles_uri();
    uri.push_str("/");
    uri.push_str(&article.id.to_string());
    uri.push_str("/edit");

    uri
}

pub fn update_article_uri(article: &Article) -> String {
    let mut uri = articles_uri();
    uri.push_str("/");
    uri.push_str(&article.id.to_string());

    uri
}

pub fn delete_article_uri(article: &Article) -> String {
    let mut uri = articles_uri();
    uri.push_str("/");
    uri.push_str(&article.id.to_string());
    uri.push_str("/delete");

    uri
}

pub fn new_article_uri() -> String {
    let mut uri = articles_uri();
    uri.push_str("/new");

    uri
}
