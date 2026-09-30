use crate::core::file::GameInfoFile;
use crate::managers::fov::FovManager;
use crate::managers::optimizations::OptimizationsManager;
use crate::shared::configs::CommandLineInterfaceConfig;
use crate::shared::traits::Manager;

use std::{ fmt::Display, io };

use anyhow::{ Error, Result };
use crossterm::{ execute, terminal::SetTitle, cursor::DisableBlinking, style::Stylize };
use inquire::{ Select, Text };

pub struct CommandLineInterface {}

impl CommandLineInterface {
	pub fn run() -> Result<()> {
		Self::init()?;

		let mut file = GameInfoFile::load()?;

		FovManager::process(&mut file)?;
		OptimizationsManager::process(&mut file)?;

		GameInfoFile::save(&file)?;

		Ok(())
	}

	fn init() -> Result<()> {
		let title = CommandLineInterfaceConfig::TITLE;
		execute!(io::stdout(), SetTitle(title), DisableBlinking)?;

		Ok(())
	}

	pub fn exit() -> Result<()> {
		let message = CommandLineInterfaceConfig::EXIT_MESSAGE;
		Text::new(message).prompt()?;

		std::process::exit(1)
	}

	pub fn error(e: Error) {
		let prefix = CommandLineInterfaceConfig::ERROR_PREFIX.red();

		println!("{prefix} {e:#}")
	}

	pub fn select_prompt<T: Display>(message: &str, options: Vec<T>) -> Result<T> {
		let answer = Select::new(message, options).with_page_size(10).prompt()?;

		Ok(answer)
	}
}
