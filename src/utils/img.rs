use anyhow::{anyhow, Error, Result};
use exif::{Exif, In, Tag};
use image::DynamicImage;

pub fn read_exif(path: &str) -> Result<Exif, Error> {
    let file = std::fs::File::open(path)?;
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();

    exifreader
        .read_from_container(&mut bufreader)
        .map_err(|e| anyhow!("error reading file: {}", e))
}

pub fn correct_orientation(mut img: DynamicImage, orientation: u32) -> DynamicImage {
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

pub fn get_orientation(exif: &Exif) -> u32 {
    match exif.get_field(Tag::Orientation, In::PRIMARY) {
        Some(orientation) => match orientation.value.get_uint(0) {
            Some(v @ 1..=8) => v,
            _ => 0,
        },
        None => 0,
    }
}
