use anyhow::{Result, anyhow};
use bon::Builder;
use promkit::{
    Prompt,
    preset::listbox::{Listbox, render::Renderer},
};

use crate::controllers::converter;

#[derive(Builder)]
struct ConverterMenu<'a> {
    items: &'a [&'a str],
}

impl ConverterMenu<'_> {
    fn prompt(&self) -> Result<Prompt<Renderer>> {
        let p = Listbox::new(self.items)
            .title("Conversion operations")
            .prompt()?;

        Ok(p)
    }
}

pub fn conversion_operations() -> Result<()> {
    loop {
        let menu = ConverterMenu::builder()
            .items(&[
                "Convert a folder of images to WebP",
                "Rename a folder of images",
                "Go back",
            ])
            .build();

        match menu.prompt()?.run()?.as_str() {
            "Convert a folder of images to WebP" => converter::encode_webp()?,
            "Rename a folder of images" => converter::rename_images()?,
            "Go back" => {
                break;
            }
            err => return Err(anyhow!("{}", err)),
        }
    }

    Ok(())
}
