use std::fs;

use anyhow::Result;
use bon::Builder;
use image::ImageReader;
use inquire::{required, CustomType};
use rayon::prelude::*;
use spinoff::{spinners, Color, Spinner};
use webp::Encoder;

#[derive(Builder)]
struct ImageConverter {
    input_folder: String,
    output_folder: String,
    quality: f32,
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
            std::fs::create_dir_all(&output_path)?;
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
