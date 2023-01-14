use std::env;

use mastodon_async::entities::prelude::Account;
use mastodon_async::helpers::toml;
use mastodon_async::prelude::*;
use mastodon_async::{helpers::cli, Result};

use crate::models::{Article, Deafie, Note, Picture};
use crate::uri_helpers::{article_uri, deafie_uri, note_uri, picture_uri};
use crate::utils::image_base_path;

async fn register() -> Result<Mastodon> {
    let server_url = env::var("MASTODON_URL").expect("env variable MASTODON_URL not set");
    let toml_path = env::var("MASTODON_TOML").expect("env variable MASTODON_TOML not set");

    let registration = Registration::new(server_url)
        .client_name("WWWTech.de")
        .scopes(Scopes::write_all() | Scopes::read_all())
        .build()
        .await?;
    let mastodon = cli::authenticate(registration).await?;

    // Save app data for using on the next run.
    toml::to_file(&mastodon.data, toml_path)?;

    Ok(mastodon)
}

pub async fn verify_or_register() -> Result<Account> {
    let toml_path = env::var("MASTODON_TOML").expect("env variable MASTODON_TOML not set");

    let mastodon = if let Ok(data) = toml::from_file(toml_path) {
        Mastodon::from(data)
    } else {
        register().await?
    };

    mastodon.verify_credentials().await
}

pub async fn post_note(note: &Note) -> Result<()> {
    let toml_path = env::var("MASTODON_TOML").expect("env variable MASTODON_TOML not set");
    let mastodon = Mastodon::from(toml::from_file(toml_path).unwrap());

    let content = format!("{} ({})", note.title, note_uri(note));

    let mut new_status = StatusBuilder::new();

    new_status
        .status(content)
        .visibility(visibility_from_str(&note.posse_visibility));

    if let Some(cw) = &note.content_warning {
        if !cw.is_empty() {
            new_status.sensitive(true).spoiler_text(cw.clone());
        }
    }

    let new_status = new_status.build()?;

    mastodon.new_status(new_status).await?;

    Ok(())
}

pub async fn post_picture(picture: &Picture) -> Result<()> {
    let toml_path = env::var("MASTODON_TOML").expect("env variable MASTODON_TOML not set");
    let mastodon = Mastodon::from(toml::from_file(toml_path).unwrap());

    let path = format!(
        "{}/{}/original/{}",
        image_base_path(),
        picture.id,
        picture.image_file_name
    );

    let attachment = mastodon.media(path, picture.alt.clone()).await?;
    let mut new_status = StatusBuilder::new();

    new_status
        .status(format!("{} ({})", picture.title, picture_uri(picture)))
        .visibility(visibility_from_str(&picture.posse_visibility))
        .media_ids([attachment.id.as_ref()]);

    if let Some(cw) = &picture.content_warning {
        if !cw.is_empty() {
            new_status.sensitive(true).spoiler_text(cw.clone());
        }
    }

    let new_status = new_status.build()?;

    mastodon.new_status(new_status).await.map_err(|e| {
        println!("Error posting status: {}", e);
        e
    })?;

    Ok(())
}

pub async fn post_article(article: &Article) -> Result<()> {
    let toml_path = env::var("MASTODON_TOML").expect("env variable MASTODON_TOML not set");
    let mastodon = Mastodon::from(toml::from_file(toml_path).unwrap());

    let content = format!("{} ({})", article.title, article_uri(article));

    let mut new_status = StatusBuilder::new();

    new_status
        .status(content)
        .visibility(visibility_from_str(&article.posse_visibility));

    if let Some(cw) = &article.content_warning {
        if !cw.is_empty() {
            new_status.sensitive(true).spoiler_text(cw.clone());
        }
    }

    let new_status = new_status.build()?;

    mastodon.new_status(new_status).await?;

    Ok(())
}

pub async fn post_deafie(deafie: &Deafie) -> Result<()> {
    let toml_path = env::var("MASTODON_TOML").expect("env variable MASTODON_TOML not set");
    let mastodon = Mastodon::from(toml::from_file(toml_path).unwrap());

    let content = format!("{} ({})", deafie.title, deafie_uri(deafie));

    let mut new_status = StatusBuilder::new();

    new_status
        .status(content)
        .visibility(visibility_from_str(&deafie.posse_visibility));

    if let Some(cw) = &deafie.content_warning {
        if !cw.is_empty() {
            new_status.sensitive(true).spoiler_text(cw.clone());
        }
    }

    let new_status = new_status.build()?;

    mastodon.new_status(new_status).await?;

    Ok(())
}

fn visibility_from_str(visiblity: &str) -> mastodon_async::Visibility {
    match visiblity {
        "public" => mastodon_async::Visibility::Public,
        "unlisted" => mastodon_async::Visibility::Unlisted,
        "private" => mastodon_async::Visibility::Private,
        "direct" => mastodon_async::Visibility::Direct,
        _ => mastodon_async::Visibility::Direct,
    }
}
