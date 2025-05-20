use anyhow::{anyhow, Result};
use bon::Builder;
use inquire::{
    ui::{Attributes, Color, RenderConfig, StyleSheet},
    Select,
};
use owo_colors::OwoColorize;

pub mod cli;
pub mod controllers;

const GREETING: &str = r#"
                _
   __________      (_)___ ___  ____ _
  / ___/ ___/_____/ / __ `__ \/ __ `/
 / /  (__  )_____/ / / / / / / /_/ /
/_/  /____/     /_/_/ /_/ /_/\__, /
                            /____/
"#;

fn get_render_cfg() -> RenderConfig<'static> {
    RenderConfig {
        answer: StyleSheet::new()
            .with_attr(Attributes::ITALIC)
            .with_fg(Color::LightCyan),
        help_message: StyleSheet::new().with_fg(Color::LightCyan),
        ..Default::default()
    }
}

#[derive(Builder)]
struct MainMenu<'a> {
    items: Vec<&'a str>,
    help_message: Option<&'a str>,
}

impl<'a> MainMenu<'a> {
    fn prompt(&self) -> Result<&'a str> {
        let choice = Select::new("What would you like to do?", self.items.clone())
            .with_help_message(self.help_message.unwrap_or_default())
            .prompt()?;

        Ok(choice)
    }
}

fn main() -> Result<()> {
    inquire::set_global_render_config(get_render_cfg());

    println!("{}", GREETING.red());
    println!("Image Processor");
    println!("By CM-IV <chuck@civdev.xyz>\n");

    loop {
        let menu = MainMenu::builder()
            .items(vec![
                "Optimize folder of images",
                "Convert folder of images to WebP",
                "Exit",
            ])
            .help_message("Main menu")
            .build();

        match menu.prompt()? {
            "Optimize folder of images" => cli::compressor_menu::compression_operations()?,
            "Convert folder of images to WebP" => cli::converter_menu::conversion_operations()?,
            "Exit" => {
                println!("{}", "\nGoodbye!\n".purple());
                break;
            }
            err => return Err(anyhow!("{}", err)),
        }
    }

    Ok(())
}
