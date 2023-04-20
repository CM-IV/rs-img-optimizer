use std::fs;

use anyhow::Result;
use webp::Encoder;
use inquire::{required, CustomType};
use spinoff::{Spinner, spinners, Color};
use rayon::prelude::*;

struct ImageConverter {
    input_folder: String,
    output_folder: String,
    quality: f32,
}

struct ImageConverterBuilder {
    input_folder: String,
    output_folder: String,
    quality: f32,
}

impl ImageConverterBuilder {
    fn new() -> Self {
        Self {
            input_folder: "".into(),
            output_folder: "".into(),
            quality: 0.0
        }
    }

    fn input_folder(mut self, input_folder: String) -> Self {
        self.input_folder = input_folder;
        self
    }

    fn output_folder(mut self, output_folder: String) -> Self {
        self.output_folder = output_folder;
        self
    }

    fn quality(mut self, quality: f32) -> Self {
        self.quality = quality;
        self
    }

    fn build(self) -> Result<ImageConverter> {
        let image = ImageConverter {
            input_folder: self.input_folder,
            output_folder: self.output_folder,
            quality: self.quality
        };

        Ok(image)
    }
}

pub fn encode_webp() -> Result<()> {
    let img = ImageConverterBuilder::new()
        .input_folder(inquire::Text::new("Enter the directory with JPG images needing conversion").with_validator(required!()).prompt()?)
        .output_folder(inquire::Text::new("Enter the output directory for the converted images").with_validator(required!()).prompt()?)
        .quality(
            CustomType::<f32>::new("What's the image quality?")
                .with_error_message("Please use a valid floating point number")
                .with_help_message("Please use numbers here")
                .prompt()?,
        )
        .build()?;

    let spinner = Spinner::new(spinners::Dots, "Converting to WebP...", Color::Yellow);

    let files = fs::read_dir(&img.input_folder)?;

    files.par_bridge().try_for_each(|file| -> Result<()> {
        let input_path = file?.path();

        let the_image = image::io::Reader::open(&input_path)?.decode()?;

        // Create the WebP encoder for the image
        let encoder: Encoder = webp::Encoder::from_image(&the_image).expect("Error encoding image");
        // Encode the image at a specified quality 0-100
        let webp: webp::WebPMemory = encoder.encode(img.quality);

        let output_path = std::path::Path::new(&img.output_folder);

        if !output_path.exists() {
            std::fs::create_dir_all(&output_path)?;
        }

        let rel_path = input_path.strip_prefix(&img.input_folder).expect("Error getting relative path");
        let final_path = output_path.join(rel_path).with_extension("webp");

        fs::write(&final_path, &*webp)?;

        Ok(())
    })?;

    spinner.success("\nDone!\n");

    Ok(())
}