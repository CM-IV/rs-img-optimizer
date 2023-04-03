use anyhow::{Result, anyhow};
use inquire::Select;

use crate::controllers::converter;

struct ConverterMenuBuilder<'a> {
    items: &'a [&'a str],
    help_message: Option<&'a str>,
}

impl<'a> ConverterMenuBuilder<'a> {
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
            "Which conversion operation would you like to perform?",
            self.items.to_vec(),
        )
        .with_help_message(self.help_message.unwrap_or_default())
        .prompt()?;

        Ok(choice)
    }
}

pub fn conversion_operations() -> Result<()> {
    loop {
        match ConverterMenuBuilder::new(&[
            "Convert a folder of JPG images to WebP",
            "Go back",
        ])
        .with_help_message("Compression menu")
        .build()?
        {
            "Convert a folder of JPG images to WebP" => converter::encode_webp()?,
            "Go back" => {
                break;
            }
            err => return Err(anyhow!("{}", err)),
        }
    }

    Ok(())
}