use std::env;

use anyhow::{anyhow, Result as AResult};
use askama::Result;
use chrono::{Duration, NaiveDate, NaiveDateTime, Utc};
use pulldown_cmark::{html, Options, Parser};

use crate::models::Note;

pub mod img;
pub mod paging;

pub static MONTHS: [&str; 12] = [
    "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
];

pub fn base_path() -> String {
    env::var("BASE_PATH").expect("env variable BASE_PATH not set")
}

pub fn image_base_path() -> String {
    env::var("IMAGE_BASE_PATH").expect("env variable IMAGE_BASE_PATH not set")
}

pub fn deafie_image_base_path() -> String {
    env::var("DEAFIE_IMAGE_BASE_PATH").expect("env variable DEAFIE_IMAGE_BASE_PATH not set")
}

pub fn static_path() -> String {
    let mut str = base_path();
    str.push_str("/static/");

    str
}

pub fn static_file_path(file: &str) -> String {
    let mut path = static_path();
    path.push_str(file.trim_start_matches('/'));
    path
}

pub fn markdown2html(md: &str) -> Result<String> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_FOOTNOTES);

    let parser = Parser::new_ext(md, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    Ok(html_output)
}

pub fn default_value(val: &Option<String>, default_val: &str) -> Result<String> {
    match val {
        Some(v) => Ok(v.clone()),
        None => Ok(default_val.to_owned()),
    }
}

pub fn time_ago_in_words(time: &NaiveDateTime) -> Result<String> {
    let now = Utc::now().naive_utc();

    let duration = now.signed_duration_since(*time);

    let words = if duration.num_days() >= 300 {
        let years = (duration.num_days() as f64) / 365.0;

        let (years, prefix) = if years.fract() > 0.75 {
            (years.ceil() as i64, "about")
        } else {
            (years.floor() as i64, "over")
        };

        if years == 1 {
            format!("{} a year ago", prefix)
        } else {
            format!("{} {} years ago", prefix, years)
        }
    } else if duration.num_days() >= 30 {
        let months = (duration.num_days() as f64) / 30.0;
        let (months, prefix) = if months.fract() > 0.75 {
            (months.ceil() as i64, "about")
        } else {
            (months.floor() as i64, "over")
        };

        if months == 1 {
            format!("{} a month ago", prefix)
        } else {
            format!("{} {} months ago", prefix, months)
        }
    } else if duration.num_hours() >= 24 {
        let days = (duration.num_hours() as f64) / 24.0;
        let (days, prefix) = if days.fract() > 0.75 {
            (days.ceil() as i64, "about")
        } else {
            (days.floor() as i64, "over")
        };

        if days == 1 {
            format!("{} a day ago", prefix)
        } else {
            format!("{} {} days ago", prefix, days)
        }
    } else if duration.num_minutes() >= 60 {
        let hours = (duration.num_minutes() as f64) / 60.0;
        let (hours, prefix) = if hours.fract() > 0.75 {
            (hours.ceil() as i64, "about")
        } else {
            (hours.floor() as i64, "over")
        };

        if hours == 1 {
            format!("{} an hour ago", prefix)
        } else {
            format!("{} {} hours ago", prefix, hours)
        }
    } else if duration.num_seconds() >= 60 {
        let minutes = (duration.num_seconds() as f64) / 60.0;
        let (minutes, prefix) = if minutes.fract() > 0.75 {
            (minutes.ceil() as i64, "about")
        } else {
            (minutes.floor() as i64, "over")
        };

        if minutes == 1 {
            format!("{} a minute ago", prefix)
        } else {
            format!("{} {} minutes ago", prefix, minutes)
        }
    } else {
        "about a minute ago".to_owned()
    };

    Ok(words)
}

pub fn date_format(date: &NaiveDateTime, format: &str) -> Result<String> {
    Ok(date.format(format).to_string())
}

pub fn link_class_by_type(note: &Note) -> Result<String> {
    let start_date = NaiveDate::from_ymd_opt(2017, 1, 19).unwrap();

    match note.note_type.as_ref() {
        "reply" => Ok("u-in-reply-to".to_owned()),
        "repost" => Ok("u-repost-of".to_owned()),
        _ => {
            if note.inserted_at.date() < start_date {
                Ok("u-in-reply-to".to_owned())
            } else {
                Ok("".to_owned())
            }
        }
    }
}

pub fn entry_class_by_type(entry_type: &str) -> Result<String> {
    match entry_type {
        "reply" => Ok("h-as-reply".to_owned()),
        "repost" => Ok("p-repost".to_owned()),
        "like" => Ok("p-like".to_owned()),
        "favorite" => Ok("p-favorite".to_owned()),
        "tag" => Ok("p-tag".to_owned()),
        "bookmark" => Ok("p-bookmark".to_owned()),
        _ => Ok("".to_owned()),
    }
}

pub fn date_list_format(date: &NaiveDateTime) -> Result<String> {
    let today = Utc::now().naive_utc().date();
    let day = date.date();

    if day == today {
        return Ok("today".to_owned());
    } else if day == (today - Duration::days(1)) {
        return Ok("yesterday".to_owned());
    }

    Ok(date.format("%Y-%m-%d").to_string())
}

pub fn content_type_from_suffix(suffix: &str) -> &str {
    match suffix {
        "image/png" => ".png",
        "image/jpg" => ".jpg",
        "image/jpeg" => ".jpg",
        "image/gif" => ".gif",
        _ => ".unknown",
    }
}

pub fn month_abbr_to_month_num(mon: &str) -> AResult<u32> {
    match mon {
        "jan" => Ok(1),
        "feb" => Ok(2),
        "mar" => Ok(3),
        "apr" => Ok(4),
        "may" => Ok(5),
        "jun" => Ok(6),
        "jul" => Ok(7),
        "aug" => Ok(8),
        "sep" => Ok(9),
        "oct" => Ok(10),
        "nov" => Ok(11),
        "dec" => Ok(12),
        _ => Err(anyhow!("month not found")),
    }
}
