use anyhow::Result;
use itertools::Itertools;
use regex::Regex;
use reqwest::{blocking::Client, header::LINK};
use url::Url;
use urlencoding::encode;
use visdom::{types::IAttrValue, Vis};

pub fn send_mentions(source_url: &str) -> Result<()> {
    let client = Client::new();
    let surl = source_url.clone();
    let body = client
        .get(surl)
        .send()
        .map_err(|e| anyhow!("could not GET source document: {}", e.to_string()))?
        .text()
        .map_err(|e| anyhow!("could not GET source document: {}", e.to_string()))?;

    let tree = Vis::load(&body).map_err(|e| anyhow!("could not parse source document: {}", e.to_string()))?;
    let links: Vec<Url> = tree
        .find("a[href]")
        .into_iter()
        .filter_map(|link| match link.get_attribute("href") {
            Some(IAttrValue::Value(val, _)) => Url::parse(&val).map_or(None, |v| Some(v)),
            _ => None,
        })
        .filter(|link| link.scheme() == "http" || link.scheme() == "https")
        .unique()
        .collect();

    for link in links.iter() {
        // we don't care about the result; if it fails, it fails 🤷‍♂️
        let _ = send_mention(&client, source_url, &link.to_string());
    }

    Ok(())
}

pub fn send_mention(client: &Client, source_url: &str, target_url: &str) -> Result<()> {
    let rsp = client
        .get(target_url)
        .send()
        .map_err(|e| anyhow!("could not get target document: {}", e.to_string()))?;
    if !rsp.status().is_success() {
        bail!("could not get target document");
    }
    let re = Regex::new(r#"rel="?webmention"?"#).unwrap();

    let mut link_hdr = rsp
        .headers()
        .get_all(LINK)
        .into_iter()
        .find(|hdr| re.is_match(hdr.to_str().unwrap()))
        .map_or(None, |v| Some(v.to_str().unwrap().to_owned()));

    if link_hdr.is_none() {
        let body = rsp.text().map_err(|_| anyhow!("could not get text body"))?;
        let tree = Vis::load(&body).map_err(|e| anyhow!("could not parse source document: {}", e.to_string()))?;
        link_hdr = match tree.find("link[rel=webmention]").attr("href") {
            Some(IAttrValue::Value(val, _)) => Some(val),
            _ => None,
        };
    }

    if link_hdr.is_none() {
        bail!("could not discover endpoint");
    }

    let link_hdr = link_hdr.unwrap();
    let re = Regex::new(r"^<|>$").unwrap();
    let re1 = Regex::new(r#";?\s*rel="?webmention"?"#).unwrap();
    let link_hdr = re1.replace_all(&link_hdr, "");
    let clean_uri = re.replace_all(&link_hdr, "");
    let url = Url::parse(&clean_uri).map_err(|_| anyhow!("could not parse endpoint URI: {}", clean_uri))?;

    let data = format!("target={}&source={}", encode(target_url), encode(source_url));

    client
        .post(url)
        .header("content-type", "application/x-www-form-urlencoded")
        .body(data)
        .send()
        .map_err(|e| anyhow!("could not send webmention: {}", e.to_string()))?;

    Ok(())
}
