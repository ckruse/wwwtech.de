use diesel::prelude::*;

use chrono::NaiveDateTime;
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
    conn: &mut PgConnection,
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

pub fn count_pictures(only_visible: bool, conn: &mut PgConnection) -> Result<i64, DbError> {
    use crate::schema::pictures::dsl::*;
    use diesel::dsl::count;

    let mut cnt_query = pictures.select(count(id)).into_boxed();

    if only_visible {
        cnt_query = cnt_query.filter(show_in_index.eq(only_visible));
    }

    let cnt = cnt_query.first::<i64>(conn)?;

    Ok(cnt)
}

pub fn get_picture(oicture_id: i32, conn: &mut PgConnection) -> Result<Picture, DbError> {
    use crate::schema::pictures::dsl::*;

    let picture = pictures.filter(id.eq(oicture_id)).first::<Picture>(conn)?;

    Ok(picture)
}

pub fn create_picture(data: &NewPicture, file: &mut File, conn: &mut PgConnection) -> Result<Picture, DbError> {
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

    conn.transaction(move |conn| {
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

        Ok(picture)
    })
}

pub fn update_picture(
    picture: &Picture,
    data: &NewPicture,
    metadata: &Option<(String, String, i32)>,
    file: &mut Option<File>,
    conn: &mut PgConnection,
) -> Result<Picture, DbError> {
    use crate::schema::pictures::dsl::*;
    use diesel::select;

    let mut data = data.clone();

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

    let now = select(diesel::dsl::now).get_result::<NaiveDateTime>(conn)?;

    let values = match metadata {
        Some((filename, content_type, len)) => (
            title.eq(data.title),
            alt.eq(data.alt),
            in_reply_to.eq(data.in_reply_to),
            lang.eq(data.lang),
            posse.eq(data.posse),
            show_in_index.eq(data.show_in_index),
            content.eq(data.content.unwrap()),
            updated_at.eq(now),
            image_file_name.eq(filename),
            image_content_type.eq(content_type),
            image_file_size.eq(len),
            image_updated_at.eq(now),
        ),
        _ => (
            title.eq(data.title),
            alt.eq(data.alt),
            in_reply_to.eq(data.in_reply_to),
            lang.eq(data.lang),
            posse.eq(data.posse),
            show_in_index.eq(data.show_in_index),
            content.eq(data.content.unwrap()),
            updated_at.eq(now),
            image_file_name.eq(&picture.image_file_name),
            image_content_type.eq(&picture.image_content_type),
            image_file_size.eq(&picture.image_file_size),
            image_updated_at.eq(picture.image_updated_at),
        ),
    };

    let picture = diesel::update(pictures.find(picture.id))
        .set(values)
        .get_result::<Picture>(conn)?;

    if let Some(file) = file {
        let path = format!(
            "{}/{}/original/{}",
            image_base_path(),
            picture.id,
            picture.image_file_name
        );

        let mut target_file = File::create(path)?;
        file.seek(std::io::SeekFrom::Start(0))?;
        std::io::copy(file, &mut target_file)?;
    }

    Ok(picture)
}

pub fn delete_picture(picture: &Picture, conn: &mut PgConnection) -> Result<usize, DbError> {
    use crate::schema::mentions;
    use crate::schema::pictures::dsl::*;

    let num_deleted = conn.transaction(move |conn| {
        diesel::delete(mentions::table.filter(mentions::picture_id.eq(picture.id))).execute(conn)?;
        diesel::delete(pictures.filter(id.eq(picture.id))).execute(conn)
    })?;

    let path = format!("{}/{}/", image_base_path(), picture.id);
    // it doesn't matter when it fails
    let _rslt = std::fs::remove_dir_all(path);

    Ok(num_deleted)
}
