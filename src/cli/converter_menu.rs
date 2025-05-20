use anyhow::{anyhow, Result};
use bon::Builder;
use inquire::Select;

use crate::controllers::converter;

#[derive(Builder)]
struct ConverterMenu<'a> {
    items: Vec<&'a str>,
    help_message: Option<&'a str>,
}

impl<'a> ConverterMenu<'a> {
    fn prompt(&self) -> Result<&'a str> {
        let choice = Select::new(
            "Which conversion operation would you like to perform?",
            self.items.clone(),
        )
        .with_help_message(self.help_message.unwrap_or_default())
        .prompt()?;

        Ok(choice)
    }
}

pub fn conversion_operations() -> Result<()> {
    loop {
        let menu = ConverterMenu::builder()
            .items(vec!["Convert a folder of images to WebP", "Go back"])
            .help_message("Conversion menu")
            .build();

        match menu.prompt()? {
            "Convert a folder of images to WebP" => converter::encode_webp()?,
            "Go back" => {
                break;
            }
            err => return Err(anyhow!("{}", err)),
        }
    }

    Ok(())
}
