use anyhow::{anyhow, Result};
use bon::Builder;
use inquire::Select;

use crate::controllers::compressor;

#[derive(Builder)]
struct CompressorMenu<'a> {
    items: Vec<&'a str>,
    help_message: Option<&'a str>,
}

impl<'a> CompressorMenu<'a> {
    fn prompt(&self) -> Result<&'a str> {
        let choice = Select::new(
            "Which compression operation would you like to perform?",
            self.items.clone(),
        )
        .with_help_message(self.help_message.unwrap_or_default())
        .prompt()?;

        Ok(choice)
    }
}

pub fn compression_operations() -> Result<()> {
    loop {
        let menu = CompressorMenu::builder()
            .items(vec!["Compress a folder of images", "Go back"])
            .help_message("Compression menu")
            .build();

        match menu.prompt()? {
            "Compress a folder of images" => compressor::compress_images()?,
            "Go back" => {
                break;
            }
            err => return Err(anyhow!("{}", err)),
        }
    }

    Ok(())
}
