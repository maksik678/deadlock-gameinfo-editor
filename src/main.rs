mod cli;
mod file;
mod fov;
mod mods;

use cli::CommandLineInterface;
use file::GameInfoFile;
use fov::FovManager;
use mods::ModsManager;

use anyhow::Result;

fn main() -> Result<()> {
	CommandLineInterface::init()?;
	let (path, file) = GameInfoFile::init()?;

	let file = FovManager::process(&file)?;
	let file = ModsManager::process(&file)?;

	GameInfoFile::save(&path, &file)?;
	CommandLineInterface::exit()?;
	
	Ok(())
}
