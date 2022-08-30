use anyhow::Result;
use chrono::naive::NaiveDateTime;
use image::imageops;
use image::GenericImageView;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::schema::deafies;
use crate::utils::correct_orientation;
use crate::utils::{deafie_image_base_path, get_orientation, read_exif};

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
pub struct Deafie {
    pub id: i32,
    pub author_id: i32,
    pub title: String,
    pub slug: String,
    pub guid: String,
    pub image_name: Option<String>,
    pub image_content_type: Option<String>,
    pub excerpt: Option<String>,
    pub body: String,
    pub published: bool,
    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Serialize, Debug, Insertable, Clone, Validate, Default)]
#[diesel(table_name = deafies)]
pub struct NewDeafie {
    pub author_id: Option<i32>,
    #[validate(length(min = 3, max = 255))]
    pub title: String,
    #[validate(length(min = 3, max = 255))]
    pub slug: String,
    pub guid: Option<String>,
    pub image_name: Option<String>,
    pub image_content_type: Option<String>,
    pub excerpt: Option<String>,
    #[validate(length(min = 3))]
    pub body: String,

    #[serde(default)]
    pub published: bool,

    pub inserted_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

const THUMB_ASPEC_RATIO: f32 = 1.0;

pub fn generate_deafie_pictures(deafie: &Deafie) -> Result<()> {
    if deafie.image_name.is_none() {
        let path = format!("{}/{}/", deafie_image_base_path(), deafie.id);
        // it doesn't matter when it fails
        let _rslt = std::fs::remove_dir_all(path);

        return Ok(());
    }

    let image_name = deafie.image_name.clone().unwrap();
    let path = format!("{}/{}/original/{}", deafie_image_base_path(), deafie.id, image_name);
    let exif = read_exif(&path)?;
    let orientation = get_orientation(&exif);

    let mut img = image::open(path)?;
    img = correct_orientation(img, orientation);

    let path = format!("{}/{}/large/{}", deafie_image_base_path(), deafie.id, image_name);
    let new_img = img.resize(800, 600, imageops::FilterType::CatmullRom);
    new_img.save(path)?;

    let path = format!("{}/{}/thumbnail/{}", deafie_image_base_path(), deafie.id, image_name);
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
    new_img.save(path)?;

    Ok(())
}
