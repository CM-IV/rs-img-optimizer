use anyhow::{Result, anyhow};
use inquire::Select;

use crate::controllers::compressor;

struct CompressorMenuBuilder<'a> {
    items: &'a [&'a str],
    help_message: Option<&'a str>,
}

impl<'a> CompressorMenuBuilder<'a> {
    fn new(items: &'a [&'a str]) -> Self {
        Self {
            items,
            help_message: None,
        }
    }

    fn with_help_message(mut self, message: &'a str) -> Self {
        self.help_message = Some(message);
        self
    }

    fn build(self) -> Result<&'a str> {
        let choice = Select::new(
            "Which compression operation would you like to perform?",
            self.items.to_vec(),
        )
        .with_help_message(self.help_message.unwrap_or_default())
        .prompt()?;

        Ok(choice)
    }
}

pub fn compression_operations() -> Result<()> {
    loop {
        match CompressorMenuBuilder::new(&[
            "Compress a folder of JPG images",
            "Go back",
        ])
        .with_help_message("Compression menu")
        .build()?
        {
            "Compress a folder of JPG images" => compressor::compress_images()?,
            "Go back" => {
                break;
            }
            err => return Err(anyhow!("{}", err)),
        }
    }

    Ok(())
}