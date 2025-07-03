#![warn(clippy::pedantic)]

use anyhow::{Result, anyhow};
use bon::Builder;
use owo_colors::OwoColorize;
use promkit::{
    Prompt,
    preset::listbox::{Listbox, render::Renderer},
};

pub mod cli;
pub mod controllers;

const GREETING: &str = r"
   __________      (_)___ ___  ____ _
  / ___/ ___/_____/ / __ `__ \/ __ `/
 / /  (__  )_____/ / / / / / / /_/ /
/_/  /____/     /_/_/ /_/ /_/\__, /
                            /____/
";

#[derive(Builder)]
struct MainMenu<'a> {
    items: &'a [&'a str],
}

impl MainMenu<'_> {
    fn prompt(&self) -> Result<Prompt<Renderer>> {
        let p = Listbox::new(self.items)
            .title("What number do you like?")
            .prompt()?;

        Ok(p)
    }
}

fn main() -> Result<()> {
    println!("{}", GREETING.red());
    println!("Image Processor");
    println!("By CM-IV <chuck@civdev.xyz>\n");

    loop {
        let menu = MainMenu::builder()
            .items(&[
                "Optimize folder of images",
                "Perform various conversion operations",
                "Exit",
            ])
            .build();

        match menu.prompt()?.run()?.as_str() {
            "Optimize folder of images" => cli::compressor_menu::compression_operations()?,
            "Perform various conversion operations" => {
                cli::converter_menu::conversion_operations()?;
            }
            "Exit" => {
                println!("{}", "\nGoodbye!\n".purple());
                break;
            }
            err => return Err(anyhow!("{}", err)),
        }
    }

    Ok(())
}
