use anyhow::Result;
use bon::Builder;
use image::ImageReader;
use inquire::{CustomType, required};
use jiff::civil::DateTime;
use little_exif::metadata::Metadata;
use rayon::prelude::*;
use spinoff::{Color, Spinner, spinners};
use std::{fs, path::PathBuf};
use webp::Encoder;

#[derive(Builder)]
struct ImageConverter {
    input_folder: String,
    output_folder: String,
    quality: f32,
}

#[derive(Builder)]
struct ImageRenamer {
    input_folder: String,
    output_folder: String,
    prefix: String,
    name: String,
}

pub fn encode_webp() -> Result<()> {
    let img = ImageConverter::builder()
        .input_folder(
            inquire::Text::new("Enter the directory with JPG images needing conversion")
                .with_validator(required!())
                .prompt()?,
        )
        .output_folder(
            inquire::Text::new("Enter the output directory for the converted images")
                .with_validator(required!())
                .prompt()?,
        )
        .quality(
            CustomType::<f32>::new("What's the image quality?")
                .with_error_message("Please use a valid floating point number")
                .with_help_message("Please use numbers here")
                .prompt()?,
        )
        .build();

    let mut spinner = Spinner::new(spinners::Dots, "Converting to WebP...", Color::Yellow);

    let files = fs::read_dir(&img.input_folder)?;

    files.par_bridge().try_for_each(|file| -> Result<()> {
        let input_path = file?.path();

        let the_image = ImageReader::open(&input_path)?.decode()?;

        let encoder: Encoder = webp::Encoder::from_image(&the_image).expect("Error encoding image");
        let webp: webp::WebPMemory = encoder.encode(img.quality);

        let output_path = std::path::Path::new(&img.output_folder);

        if !output_path.exists() {
            std::fs::create_dir_all(output_path)?;
        }

        let rel_path = input_path
            .strip_prefix(&img.input_folder)
            .expect("Error getting relative path");
        let final_path = output_path.join(rel_path).with_extension("webp");

        fs::write(&final_path, &*webp)?;

        Ok(())
    })?;

    spinner.success("\nDone!\n");

    Ok(())
}

pub fn rename_images() -> Result<()> {
    let renamer = ImageRenamer::builder()
        .input_folder(
            inquire::Text::new("Enter the directory with images to rename: ")
                .with_validator(required!())
                .prompt()?,
        )
        .output_folder(
            inquire::Text::new("Enter the output directory for the renamed images: ")
                .with_validator(required!())
                .prompt()?,
        )
        .prefix(
            inquire::Text::new("Enter a prefix for the renamed files: ")
                .with_validator(required!())
                .prompt()?,
        )
        .name(
            inquire::Text::new("Enter the new file name: ")
                .with_validator(required!())
                .prompt()?,
        )
        .build();

    let mut spinner = Spinner::new(spinners::Dots, "Renaming images...", Color::Yellow);

    let files = fs::read_dir(&renamer.input_folder)?;

    files.par_bridge().try_for_each(|file| -> Result<()> {
        let input_path = file?.path();

        let metadata = Metadata::new_from_path(&input_path)?;

        let datetime = metadata
            .get_tag(&little_exif::exif_tag::ExifTag::DateTimeOriginal(
                String::new(),
            ))
            .next()
            .and_then(|tag| {
                let bytes = tag.value_as_u8_vec(&little_exif::endian::Endian::Little);
                let s = str::from_utf8(&bytes).ok()?.trim_matches('\0');

                // Convert "YYYY:MM:DD HH:MM:SS" to "YYYY-MM-DD HH:MM:SS"
                let parts: Vec<&str> = s.splitn(2, ' ').collect();
                if parts.len() == 2 {
                    let date_part = parts[0].replace(':', "-");
                    let time_part = parts[1];
                    let formatted = format!("{date_part} {time_part}");
                    let datetime = formatted.parse::<DateTime>().expect("DateTime");
                    let chicago_time = datetime.in_tz("America/Chicago").expect("Timestamp");
                    Some((
                        chicago_time.strftime("%y_%m"),       // For grouping
                        chicago_time.strftime("%d_%H_%M_%S"), // For uniqueness
                    ))
                } else {
                    None
                }
            });

        let new_name = match datetime {
            Some((month_year, unique_part)) => {
                format!(
                    "{}{}_{}_{}",
                    renamer.prefix, month_year, unique_part, renamer.name
                )
            }
            None => input_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("image")
                .to_string(),
        };

        let output_path = PathBuf::from(&renamer.output_folder);
        if !output_path.exists() {
            std::fs::create_dir_all(&output_path)?;
        }

        let extension = input_path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        let final_path = output_path.join(format!("{new_name}.{extension}"));

        fs::copy(&input_path, &final_path)?;

        Ok(())
    })?;

    spinner.success("\nDone!\n");

    Ok(())
}
