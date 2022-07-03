use chrono::naive::NaiveDateTime;
use exif::{Exif, In, Tag};
use serde::{Deserialize, Serialize};
use validator::Validate;

use anyhow::{Error, Result};
use image::GenericImageView;
use image::{imageops, DynamicImage};

use crate::schema::pictures;
use crate::utils::image_base_path;

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
pub struct Picture {
    pub id: i32,
    pub author_id: i32,

    pub in_reply_to: Option<String>,
    pub webmentions_count: i32,

    pub image_file_name: String,
    pub image_content_type: String,
    pub image_file_size: i32,
    pub image_updated_at: NaiveDateTime,

    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,

    pub title: String,

    pub posse: bool,
    pub show_in_index: bool,

    pub content: String,

    pub lang: String,
    pub alt: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Insertable, Clone, Validate, Default)]
#[table_name = "pictures"]
pub struct NewPicture {
    pub author_id: Option<i32>,
    #[validate(length(min = 5))]
    pub title: String,
    #[validate(length(min = 5))]
    pub alt: Option<String>,
    #[validate(url)]
    pub in_reply_to: Option<String>,
    #[validate(length(min = 2, max = 2))]
    pub lang: String,
    #[serde(default)]
    pub posse: bool,
    #[serde(default)]
    pub show_in_index: bool,
    #[validate(required, length(min = 5))]
    pub content: Option<String>,

    pub image_file_name: Option<String>,
    pub image_content_type: Option<String>,
    pub image_file_size: Option<i32>,
    pub image_updated_at: Option<NaiveDateTime>,

    pub inserted_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewJsonPicture {
    #[serde(flatten)]
    pub new_picture: NewPicture,
    pub picture: Option<String>,
}

const THUMB_ASPEC_RATIO: f32 = 1.0;

fn read_exif(path: &str) -> Result<Exif, Error> {
    let file = std::fs::File::open(path)?;
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();

    Ok(exifreader
        .read_from_container(&mut bufreader)
        .map_err(|_| anyhow!("error reading file"))?)
}

fn correct_orientation(mut img: DynamicImage, orientation: u32) -> DynamicImage {
    if orientation <= 1 || orientation > 8 {
        return img;
    }

    if orientation >= 5 {
        img = img.rotate90().fliph();
    }

    if orientation == 3 || orientation == 4 || orientation == 7 || orientation == 8 {
        img = img.rotate180();
    }

    if orientation % 2 == 0 {
        img = img.fliph();
    }

    img
}

pub fn generate_pictures(picture: &Picture) -> Result<()> {
    let path = format!(
        "{}/{}/original/{}",
        image_base_path(),
        picture.id,
        picture.image_file_name
    );
    let exif = read_exif(&path)?;

    let orientation = match exif.get_field(Tag::Orientation, In::PRIMARY) {
        Some(orientation) => match orientation.value.get_uint(0) {
            Some(v @ 1..=8) => v,
            _ => 0,
        },
        None => 0,
    };

    let orig_path = path.clone();
    let mut img = image::open(path)?;

    if !orig_path.ends_with(".png") && !orig_path.ends_with(".gif") {
        img.save_with_format(orig_path, image::ImageFormat::Jpeg)?;
    }

    img = correct_orientation(img, orientation);

    let path = format!("{}/{}/large/{}", image_base_path(), picture.id, picture.image_file_name);
    let new_img = img.resize(800, 600, imageops::FilterType::CatmullRom);
    if !path.ends_with(".png") && !path.ends_with(".gif") {
        new_img.save_with_format(path, image::ImageFormat::Jpeg)?;
    } else {
        new_img.save(path)?;
    }

    let path = format!(
        "{}/{}/thumbnail/{}",
        image_base_path(),
        picture.id,
        picture.image_file_name
    );
    let (width, height) = img.dimensions();
    let aspect_ratio = width as f32 / height as f32;

    let img = if aspect_ratio != THUMB_ASPEC_RATIO {
        let mid_x = width / 2;
        let mid_y = height / 2;

        if width > height {
            img.crop(mid_x - (height / 2), mid_y - (height / 2), height, height)
        } else {
            img.crop(mid_x - (width / 2), mid_y - (width / 2), width, width)
        }
    } else {
        img
    };

    let new_img = img.resize_exact(600, 600, imageops::FilterType::CatmullRom);

    if !path.ends_with(".png") && !path.ends_with(".gif") {
        new_img.save_with_format(path, image::ImageFormat::Jpeg)?;
    } else {
        new_img.save(path)?;
    }

    Ok(())
}
