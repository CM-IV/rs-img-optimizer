use owo_colors::OwoColorize;
use anyhow::{Result, anyhow};
use inquire::{
    ui::{Attributes, Color, RenderConfig, StyleSheet},
    Select,
};

pub mod controllers;
pub mod cli;

fn get_render_cfg() -> RenderConfig {
    RenderConfig {
        answer: StyleSheet::new()
            .with_attr(Attributes::ITALIC)
            .with_fg(Color::LightCyan),
        help_message: StyleSheet::new().with_fg(Color::LightCyan),
        ..Default::default()
    }
}

struct MainMenuBuilder<'a> {
    items: &'a [&'a str],
    help_message: Option<&'a str>,
}

impl<'a> MainMenuBuilder<'a> {
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
        let choice = Select::new("What would you like to do?", self.items.to_vec())
            .with_help_message(self.help_message.unwrap_or_default())
            .prompt()?;

        Ok(choice)
    }
}

fn main() -> Result<()> {
    inquire::set_global_render_config(get_render_cfg());

    let greet = r#"
                    _             
   __________      (_)___ ___  ____ _
  / ___/ ___/_____/ / __ `__ \/ __ `/
 / /  (__  )_____/ / / / / / / /_/ / 
/_/  /____/     /_/_/ /_/ /_/\__, /  
                            /____/                                                                     
    "#;

    println!("{}", greet.red());
    println!("Image Processor");
    println!("By CM-IV <chuck@civdev.xyz>\n");

    loop {
        match MainMenuBuilder::new(&[
            "Optimize folder of images",
            "Convert folder of images to WebP",
            "Exit",
        ])
        .with_help_message("Main menu")
        .build()?
        {
            "Optimize JPG images" => cli::compressor_menu::compression_operations()?,
            "Convert JPG images" => cli::converter_menu::conversion_operations()?,
            "Exit" => {
                println!("{}", "\nGoodbye!\n".purple());
                break;
            }
            err => return Err(anyhow!("{}", err)),
        }
    }

    Ok(())
}