use crate::shared::configs::CommandLineInterfaceConfig;

use std::{ fmt::Display, io };

use anyhow::Result;
use crossterm::{ execute, terminal::SetTitle, cursor::DisableBlinking };
use inquire::{ Select, Text };

pub struct CommandLineInterface {}

impl CommandLineInterface {
	pub fn init() -> Result<()> {
		let title = CommandLineInterfaceConfig::TITLE;
		execute!(io::stdout(), SetTitle(&title), DisableBlinking)?;

		Ok(())
	}

	pub fn exit() -> Result<()> {
		let message = CommandLineInterfaceConfig::EXIT_MESSAGE;
		Text::new(message).prompt()?;

		Ok(())
	}

	pub fn select_prompt<T: Display>(message: &str, options: Vec<T>) -> Result<T> {
		let answer = Select::new(message, options).with_page_size(10).prompt()?;

		Ok(answer)
	}
}
