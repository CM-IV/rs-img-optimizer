use owo_colors::OwoColorize;
use anyhow::Result;
use inquire::CustomType;
use inquire::required;
use image_compressor::FolderCompressor;
use image_compressor::Factor;
use spinoff::{Spinner, spinners, Color};

use std::sync::mpsc;


struct ImageCompressor {
    input_folder: String,
    output_folder: String,
    quality: f32,
}

struct ImageCompressorBuilder {
    input_folder: String,
    output_folder: String,
    quality: f32,
}

impl ImageCompressorBuilder {
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

    fn build(self) -> Result<ImageCompressor> {
        let image = ImageCompressor {
            input_folder: self.input_folder,
            output_folder: self.output_folder,
            quality: self.quality
        };

        Ok(image)
    }
}

pub fn compress_images() -> Result<()> {

    let num = num_cpus::get() as u32;
    let (tx, _tr) = mpsc::channel();  


    let img = ImageCompressorBuilder::new()
        .input_folder(inquire::Text::new("Enter the directory containing JPG images").with_validator(required!()).prompt()?)
        .output_folder(inquire::Text::new("Enter the output directory for the compressed images").with_validator(required!()).prompt()?)
        .quality(
            CustomType::<f32>::new("What's the image quality?")
                .with_error_message("Please use a valid floating point number")
                .with_help_message("Please use numbers here")
                .prompt()?,
        )
        .build()?;

    let spinner = Spinner::new(spinners::Dots, "Compressing...", Color::Yellow);

    let mut comp = FolderCompressor::new(img.input_folder, img.output_folder);
    comp.set_factor(Factor::new(img.quality, 1.0));
    comp.set_thread_count(num);
    comp.set_sender(tx);

    match comp.compress(){
        Ok(_) => {
            spinner.success("\nDone!\n");
        },
        Err(_) => {
            println!("{}", "\nError! Cannot compress the images!".red());
            return Ok(());
        }
    }

    Ok(())
}