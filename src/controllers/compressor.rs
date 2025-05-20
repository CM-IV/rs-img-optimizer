use anyhow::Result;
use bon::Builder;
use image_compressor::Factor;
use image_compressor::FolderCompressor;
use inquire::required;
use inquire::CustomType;
use owo_colors::OwoColorize;
use spinoff::{spinners, Color, Spinner};

use std::sync::mpsc;

#[derive(Builder)]
struct ImageCompressor {
    input_folder: String,
    output_folder: String,
    quality: f32,
}

pub fn compress_images() -> Result<()> {
    let num = num_cpus::get() as u32;
    let (tx, _tr) = mpsc::channel();

    let img = ImageCompressor::builder()
        .input_folder(
            inquire::Text::new("Enter the directory containing JPG images")
                .with_validator(required!())
                .prompt()?,
        )
        .output_folder(
            inquire::Text::new("Enter the output directory for the compressed images")
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

    let mut spinner = Spinner::new(spinners::Dots, "Compressing...", Color::Yellow);

    let mut comp = FolderCompressor::new(img.input_folder, img.output_folder);
    comp.set_factor(Factor::new(img.quality, 1.0));
    comp.set_thread_count(num);
    comp.set_sender(tx);

    match comp.compress() {
        Ok(_) => {
            spinner.success("\nDone!\n");
        }
        Err(_) => {
            println!("{}", "\nError! Cannot compress the images!".red());
            return Ok(());
        }
    }

    Ok(())
}
