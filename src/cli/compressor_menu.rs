use anyhow::{Result, anyhow};
use bon::Builder;
use promkit::{
    Prompt,
    preset::listbox::{Listbox, render::Renderer},
};

use crate::controllers::compressor;

#[derive(Builder)]
struct CompressorMenu<'a> {
    items: &'a [&'a str],
}

impl CompressorMenu<'_> {
    fn prompt(&self) -> Result<Prompt<Renderer>> {
        let p = Listbox::new(self.items)
            .title("Compression operations")
            .prompt()?;

        Ok(p)
    }
}

pub fn compression_operations() -> Result<()> {
    loop {
        let menu = CompressorMenu::builder()
            .items(&["Compress a folder of images", "Go back"])
            .build();

        match menu.prompt()?.run()?.as_str() {
            "Compress a folder of images" => compressor::compress_images()?,
            "Go back" => {
                break;
            }
            err => return Err(anyhow!("{}", err)),
        }
    }

    Ok(())
}
