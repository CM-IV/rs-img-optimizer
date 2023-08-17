use std::{sync::mpsc, fs};

use camino::Utf8PathBuf;
use flemish::{Sandbox, OnEvent, Settings, color_themes};
use anyhow::Result;
use fltk::{
    button::*,
    dialog,
    group::{Group, Pack, Tabs, self},
    prelude::{GroupExt, WidgetBase, WidgetExt, ValuatorExt}, frame, valuator,
};
use image_compressor::{FolderCompressor, Factor};
use rayon::prelude::{ParallelBridge, ParallelIterator};
use webp::Encoder;

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


#[derive(Default)]
pub struct ImgOptimizer;

#[derive(Debug, Clone)]
pub enum Message {
    Compress {
        quality: f32
    },
    Convert {
        quality: f32
    }
}

impl Sandbox for ImgOptimizer {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Rust Image Optimizer")
    }

    fn update(&mut self, message: Self::Message) {

        match message {
            Message::Compress { quality } => {

                let picture_dir = dirs::picture_dir().unwrap();
                let picture_str = Utf8PathBuf::from_path_buf(picture_dir).expect("can't convert to Utf8PathBuf");
                let out_folder: Utf8PathBuf = format!("{picture_str}/comp").into();

                let num = num_cpus::get() as u32;
                let (tx, _tr) = mpsc::channel();

                let mut dialog = dialog::NativeFileChooser::new(dialog::NativeFileChooserType::BrowseDir);
                dialog.set_title("folder of JPGs");
                dialog.show();

                let binding = dialog.filename();

                if binding.is_dir() {
                    let picked = Utf8PathBuf::from_path_buf(binding).expect("Could not get path to folder");

                    let img = ImageCompressorBuilder::new()
                        .input_folder(picked)
                        .output_folder(out_folder)
                        .quality(quality)
                        .build()
                        .unwrap();

                    let mut comp = FolderCompressor::new(img.input_folder, img.output_folder);
                    comp.set_factor(Factor::new(img.quality, 1.0));
                    comp.set_thread_count(num / 2);
                    comp.set_sender(tx);

                    if comp.compress().is_ok() {
                        dialog::message_title("Success!");
                        dialog::message_default("Images successfully compressed");
                    } else {
                        dialog::message_title("Error!");
                        dialog::alert_default("There was an error, the images were not compressed");
                    }
                }
            },
            Message::Convert { quality } => {

                let picture_dir = dirs::picture_dir().unwrap();
                let picture_str = Utf8PathBuf::from_path_buf(picture_dir).expect("can't convert to Utf8PathBuf");
                let out_folder: Utf8PathBuf = format!("{picture_str}/webps").into();

                let mut dialog = dialog::NativeFileChooser::new(dialog::NativeFileChooserType::BrowseDir);
                dialog.set_title("folder of JPGs");
                dialog.show();
                let binding = dialog.filename();
                if binding.is_dir() {
                    let picked = Utf8PathBuf::from_path_buf(binding).expect("Could not get path to folder");

                    let img = ImageConverterBuilder::new()
                        .input_folder(picked)
                        .output_folder(out_folder)
                        .quality(quality)
                        .build()
                        .unwrap();

                    let files = fs::read_dir(&img.input_folder).expect("Couldn't read input dir");

                    let res = files.par_bridge().try_for_each(|file| -> Result<()> {
                        let input_path = file?.path();
                
                        let the_image = image::io::Reader::open(&input_path)?.decode()?;
                
                        // Create the WebP encoder for the image
                        let encoder: Encoder = webp::Encoder::from_image(&the_image).expect("Error encoding image");
                        // Encode the image at a specified quality 0-100
                        let webp: webp::WebPMemory = encoder.encode(img.quality);
                
                        let output_path = std::path::Path::new(&img.output_folder);
                
                        if !output_path.exists() {
                            std::fs::create_dir_all(output_path)?;
                        }
                
                        let rel_path = input_path.strip_prefix(&img.input_folder).expect("Error getting relative path");
                        let final_path = output_path.join(rel_path).with_extension("webp");
                
                        fs::write(final_path, &*webp)?;
                
                        Ok(())
                    });

                    if res.is_ok() {
                        dialog::message_title("Success!");
                        dialog::message_default("Images successfully converted to WebP");
                    } else {
                        dialog::message_title("Error!");
                        dialog::alert_default("There was an error, the images were not converted");
                    };
                }
            }
        }
    }

    fn view(&mut self) {
        // vv Draw the interface vv

        let tab = Tabs::new(10, 10, 700 - 20, 450 - 20, "");

        let grp1 = Group::new(10, 35, 700 - 20, 450 - 45, "Compress\t\t");

        let mut pack = Pack::new(215, 150, 250, 450 - 45, None);
        pack.set_spacing(10);
        let flex = group::Flex::default()
            .with_size(250, 100)
            .column()
            .center_of_parent();
        frame::Frame::default().with_label("Photo Quality");
        let mut slider1 = valuator::HorValueSlider::default().with_size(250, 20).center_of_parent();
        slider1.set_minimum(0.);
        slider1.set_maximum(100.);
        slider1.set_step(1., 1); // increment by 1.0 at each 1 step
        slider1.set_value(80.); // start at 80%
        flex.end();
        Button::default()
            .with_size(80, 30)
            .with_label("Select folder")
            .on_event(Message::Compress { quality: slider1.value() as f32 });

        pack.end();
        grp1.end();

        let grp2 = Group::new(10, 35, 700 - 20, 450 - 45, "Convert\t\t");
        let mut pack = Pack::new(215, 150, 250, 450 - 45, None);
        pack.set_spacing(10);
        let flex = group::Flex::default()
            .with_size(250, 100)
            .column()
            .center_of_parent();
        frame::Frame::default().with_label("Photo Quality");
        let mut slider2 = valuator::HorValueSlider::default().with_size(250, 20).center_of_parent();
        slider2.set_minimum(0.);
        slider2.set_maximum(100.);
        slider2.set_step(1., 1); // increment by 1.0 at each 1 step
        slider2.set_value(80.); // start at 80%
        flex.end();
        Button::default()
            .with_size(80, 30)
            .with_label("Select folder")
            .on_event(Message::Convert { quality: slider2.value() as f32 });

        pack.end();
        grp2.end();
        tab.end();
        // ^^ Draw the interface ^^
    }
}

fn main() {
    ImgOptimizer::new().run(Settings {
        size: (700, 450),
        resizable: false,
        color_map: Some(color_themes::DARK_THEME),
        ..Default::default()
    })
}