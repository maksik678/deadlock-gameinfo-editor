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
	let (path, file) = GameInfoFile::init()?;

	let file = FovManager::process(&file)?;
	let file = MinionHealthManager::process(&file)?;
	let file = PlayerHealthManager::process(&file)?;
	let file = ModsManager::process(&file)?;

	GameInfoFile::save(&path, &file)?;
	CommandLineInterface::exit()?;
	
	Ok(())
}
