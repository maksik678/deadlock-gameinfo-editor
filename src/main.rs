mod cli;
mod file;
mod fov;
mod health_minion;
mod health_player;
mod mods;

use cli::CommandLineInterface;
use file::GameInfoFile;
use fov::FovManager;
use health_minion::MinionHealthManager;
use health_player::PlayerHealthManager;
use mods::ModsManager;

use anyhow::Result;

fn main() -> Result<()> {
	CommandLineInterface::init()?;
	let mut file = GameInfoFile::init()?;

	FovManager::process(&mut file)?;
	MinionHealthManager::process(&mut file)?;
	PlayerHealthManager::process(&mut file)?;
	ModsManager::process(&mut file)?;

	GameInfoFile::save(&mut file)?;
	CommandLineInterface::exit()?;

	Ok(())
}
