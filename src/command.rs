use std::{sync::mpsc, fs};

use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};
use anyhow::Result;
use image_compressor::{FolderCompressor, Factor};
use owo_colors::OwoColorize;
use rayon::prelude::{ParallelBridge, ParallelIterator};
use spinoff::{Spinner, spinners};
use webp::Encoder;

#[derive(Parser)]
#[clap(
    author = "CM-IV <chuck@civdev.xyz>",
    version,
    long_about = r#"
Image Processing Software
By CM-IV <chuck@civdev.xyz>
"#
)]
pub struct ImgOptimizer {
    #[clap(subcommand, value_enum)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Compress folder of JPGs
    Compress {
        /// Required quality for resultant images
        #[arg(short, long)]
        quality: f32,

        /// The path to the folder
        path: Utf8PathBuf
    },
    /// Convert folder of JPGs to WebP
    Convert {
        /// Required quality for resultant images
        #[arg(short, long)]
        quality: f32,

        /// The path to the folder
        path: Utf8PathBuf
    }
}

struct ImageCompressor {
    input_folder: Utf8PathBuf,
    output_folder: Utf8PathBuf,
    quality: f32,
}

struct ImageCompressorBuilder {
    input_folder: Utf8PathBuf,
    output_folder: Utf8PathBuf,
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

    fn input_folder(mut self, input_folder: Utf8PathBuf) -> Self {
        self.input_folder = input_folder;
        self
    }

    fn output_folder(mut self, output_folder: Utf8PathBuf) -> Self {
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

struct ImageConverter {
    input_folder: Utf8PathBuf,
    output_folder: Utf8PathBuf,
    quality: f32,
}

struct ImageConverterBuilder {
    input_folder: Utf8PathBuf,
    output_folder: Utf8PathBuf,
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

    fn input_folder(mut self, input_folder: Utf8PathBuf) -> Self {
        self.input_folder = input_folder;
        self
    }

    fn output_folder(mut self, output_folder: Utf8PathBuf) -> Self {
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

impl ImgOptimizer {
    fn compress_images(folder: Utf8PathBuf, quality: f32) -> Result<()> {

        let picture_dir = dirs::picture_dir().unwrap();
        let picture_str = Utf8PathBuf::from_path_buf(picture_dir).expect("can't convert to Utf8PathBuf");
        let out_folder: Utf8PathBuf = format!("{picture_str}/comp").into();

        let num = num_cpus::get() as u32;
        let (tx, _tr) = mpsc::channel();  
    
    
        let img = ImageCompressorBuilder::new()
            .input_folder(folder)
            .output_folder(out_folder)
            .quality(quality)
            .build()?;
    
        let spinner = Spinner::new(spinners::Dots, "Compressing...", spinoff::Color::Yellow);
    
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
    fn encode_webp(folder: Utf8PathBuf, quality: f32) -> Result<()> {

        let picture_dir = dirs::picture_dir().unwrap();
        let picture_str = Utf8PathBuf::from_path_buf(picture_dir).expect("can't convert to Utf8PathBuf");
        let out_folder: Utf8PathBuf = format!("{picture_str}/webps").into();

        let img = ImageConverterBuilder::new()
            .input_folder(folder)
            .output_folder(out_folder)
            .quality(quality)
            .build()?;
    
        let spinner = Spinner::new(spinners::Dots, "Converting to WebP...", spinoff::Color::Yellow);
    
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
    pub fn exec(self) -> Result<()> {
        match self.command {
            Command::Compress { path, quality } => {

                if !path.is_dir() {
                    println!("{}", "\nYou cannot use two commands at once and the path must lead to a directory\n".red());
                    return Ok(());
                }

                Self::compress_images(path, quality)?;
            },
            Command::Convert { path, quality } => {

                if !path.is_dir() {
                    println!("{}", "\nYou cannot use two commands at once and the path must lead to a directory\n".red());
                    return Ok(());
                }

                let result = Self::encode_webp(path, quality);

                match result {
                    Ok(()) => {
                        return Ok(());
                    },
                    Err(_) => {
                        println!("{}", "\nAn error occured during conversion\n".red());
                        return Ok(());
                    }
                }
            }
        }

        Ok(())

    }
}