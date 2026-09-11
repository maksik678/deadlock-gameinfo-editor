mod core {
	pub mod cli;
	pub mod file;
}
mod managers {
	pub mod fov;
	pub mod health_minion;
	pub mod health_player;
	pub mod optimizations;
}

mod shared {
	pub mod configs;
	pub mod enums;
	pub mod traits;
}

use crate::core::cli::CommandLineInterface;
use crate::core::file::GameInfoFile;
use crate::managers::fov::FovManager;
use crate::managers::health_minion::MinionHealthManager;
use crate::managers::health_player::PlayerHealthManager;
use crate::managers::optimizations::OptimizationsManager;
use crate::shared::traits::Manager;

use anyhow::Result;

fn main() -> Result<()> {
	CommandLineInterface::init()?;
	let mut file = GameInfoFile::init()?;

	FovManager::process(&mut file)?;
	MinionHealthManager::process(&mut file)?;
	PlayerHealthManager::process(&mut file)?;
	OptimizationsManager::process(&mut file)?;

	GameInfoFile::save(&mut file)?;
	CommandLineInterface::exit()?;

	Ok(())
}
