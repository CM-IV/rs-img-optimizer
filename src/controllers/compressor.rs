use anyhow::Result;
use bon::Builder;
use image_compressor::Factor;
use image_compressor::FolderCompressor;
use owo_colors::OwoColorize;
use promkit::preset::listbox::Listbox;
use promkit::preset::readline::Readline;
use spinoff::{Color, Spinner, spinners};

#[derive(Builder)]
struct ImageCompressor {
    input_folder: String,
    output_folder: String,
    quality: f32,
}

pub fn compress_images() -> Result<()> {
    let num = num_cpus::get() as u32;

    let img = ImageCompressor::builder()
        .input_folder(
            Readline::default()
                .title("Enter the input directory for the images to be compressed")
                .prompt()?
                .run()?,
        )
        .output_folder(
            Readline::default()
                .title("Enter the output directory for the compressed images")
                .prompt()?
                .run()?,
        )
        .quality(
            Listbox::new(0..100)
                .title("What's the image quality?")
                .prompt()?
                .run()?
                .parse::<f32>()?,
        )
        .build();

    let mut spinner = Spinner::new(spinners::Dots, "Compressing...", Color::Yellow);

    let mut comp = FolderCompressor::new(img.input_folder, img.output_folder);
    comp.set_factor(Factor::new(img.quality, 1.0));
    comp.set_thread_count(num);

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
