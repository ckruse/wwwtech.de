use diesel::prelude::*;

use image::imageops::FilterType;

use chrono::NaiveDateTime;
use image::{GenericImageView, ImageError};
use std::fs::File;
use std::io::Seek;
use std::vec::Vec;
use validator::Validate;

use crate::models::{NewPicture, Picture};
use crate::utils::image_base_path;
use crate::DbError;

pub fn list_pictures(
    limit: i64,
    offset: i64,
    only_visible: bool,
    conn: &PgConnection,
) -> Result<Vec<Picture>, DbError> {
    use crate::schema::pictures::dsl::*;

    let mut pictures_list_query = pictures
        .order_by(inserted_at.desc())
        .then_order_by(updated_at.desc())
        .then_order_by(id.desc())
        .limit(limit)
        .offset(offset)
        .into_boxed();

    if only_visible {
        pictures_list_query = pictures_list_query.filter(show_in_index.eq(only_visible));
    }

    let pictures_list = pictures_list_query.load::<Picture>(conn)?;

    Ok(pictures_list)
}

pub fn count_pictures(only_visible: bool, conn: &PgConnection) -> Result<i64, DbError> {
    use crate::schema::pictures::dsl::*;
    use diesel::dsl::count;

    let mut cnt_query = pictures.select(count(id)).into_boxed();

    if only_visible {
        cnt_query = cnt_query.filter(show_in_index.eq(only_visible));
    }

    let cnt = cnt_query.first::<i64>(conn)?;

    Ok(cnt)
}

pub fn get_picture(oicture_id: i32, conn: &PgConnection) -> Result<Picture, DbError> {
    use crate::schema::pictures::dsl::*;

    let picture = pictures.filter(id.eq(oicture_id)).first::<Picture>(conn)?;

    Ok(picture)
}

pub fn create_picture(data: &NewPicture, file: &mut File, conn: &PgConnection) -> Result<Picture, DbError> {
    use crate::schema::pictures;
    use diesel::select;

    let now = select(diesel::dsl::now).get_result::<NaiveDateTime>(conn)?;
    let mut data = data.clone();
    data.inserted_at = Some(now);
    data.updated_at = Some(now);
    data.image_updated_at = Some(now);

    if data.in_reply_to == Some("".to_owned()) {
        data.in_reply_to = None;
    }

    if data.alt == Some("".to_owned()) {
        data.alt = None;
    }

    if data.content.is_none() || data.content == Some("".to_owned()) {
        data.content = Some(data.title.clone());
    }

    data.validate()?;

    conn.transaction(move || {
        let picture = diesel::insert_into(pictures::table)
            .values(data)
            .get_result::<Picture>(conn)?;

        let path = format!("{}/{}/original", image_base_path(), picture.id);
        std::fs::create_dir_all(path)?;

        let path = format!("{}/{}/large", image_base_path(), picture.id);
        std::fs::create_dir_all(path)?;

        let path = format!("{}/{}/thumbnail", image_base_path(), picture.id);
        std::fs::create_dir_all(path)?;

        let path = format!(
            "{}/{}/original/{}",
            image_base_path(),
            picture.id,
            picture.image_file_name
        );

        let mut target_file = File::create(path)?;
        file.seek(std::io::SeekFrom::Start(0))?;
        std::io::copy(file, &mut target_file)?;

        let _rslt = create_images(&picture);

        Ok(picture)
    })
}

const THUMB_ASPEC_RATIO: f32 = 1.0;

fn create_images(picture: &Picture) -> Result<(), ImageError> {
    let path = format!(
        "{}/{}/original/{}",
        image_base_path(),
        picture.id,
        picture.image_file_name
    );

    let mut img = image::open(path)?;

    let path = format!("{}/{}/large/{}", image_base_path(), picture.id, picture.image_file_name);
    let new_img = img.resize(800, 600, FilterType::CatmullRom);
    new_img.save(path)?;

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

    let new_img = img.resize_exact(600, 600, FilterType::CatmullRom);
    new_img.save(path)?;

    Ok(())
}

pub fn delete_picture(picture: &Picture, conn: &PgConnection) -> Result<usize, DbError> {
    use crate::schema::pictures::dsl::*;

    let num_deleted = diesel::delete(pictures.filter(id.eq(picture.id))).execute(conn)?;

    let path = format!("{}/{}/", image_base_path(), picture.id);
    // it doesn't matter when it fails
    let _rslt = std::fs::remove_dir_all(path);

    Ok(num_deleted)
}
