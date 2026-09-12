use crate::core::file::GameInfoFile;
use crate::managers::fov::FovManager;
use crate::managers::health_minion::MinionHealthManager;
use crate::managers::health_player::PlayerHealthManager;
use crate::managers::optimizations::OptimizationsManager;
use crate::shared::traits::Manager;
use crate::shared::configs::CommandLineInterfaceConfig;

use std::{ fmt::Display, io };

use anyhow::{ Error, Result };
use crossterm::{ execute, terminal::SetTitle, cursor::DisableBlinking, style::Stylize };
use inquire::{ Select, Text };

pub struct CommandLineInterface {}

impl CommandLineInterface {
	pub fn init() -> Result<()> {
		let title = CommandLineInterfaceConfig::TITLE;
		execute!(io::stdout(), SetTitle(&title), DisableBlinking)?;

		Ok(())
	}

	pub fn run() -> Result<()> {
		let mut file = GameInfoFile::init()?;

		FovManager::process(&mut file)?;
		MinionHealthManager::process(&mut file)?;
		PlayerHealthManager::process(&mut file)?;
		OptimizationsManager::process(&mut file)?;

		GameInfoFile::save(&mut file)?;

		Ok(())
	}

	pub fn error(e: Error) -> Result<()> {
		let prefix = format!("Error:").red();
		println!("{prefix} {e:#}");

		Ok(())
	}

	pub fn exit() -> Result<()> {
		let message = CommandLineInterfaceConfig::EXIT_MESSAGE;
		Text::new(message).prompt()?;
		std::process::exit(1)
	}

	pub fn select_prompt<T: Display>(message: &str, options: Vec<T>) -> Result<T> {
		let answer = Select::new(message, options).with_page_size(10).prompt()?;

		Ok(answer)
	}
}
