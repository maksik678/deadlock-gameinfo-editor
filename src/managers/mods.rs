use crate::core::cli::CommandLineInterface;
use crate::core::file::GameInfoFile;
use crate::shared::configs::ModsConfig;
use crate::shared::enums::Options;

use anyhow::Result;
use strum::IntoEnumIterator;

pub struct ModsManager;

impl ModsManager {
	pub fn process(file: &mut GameInfoFile) -> Result<()> {
		let message = ModsConfig::PROMPT_MESSAGE;
		let options = Options::iter().collect();

		let answer = CommandLineInterface::select_prompt(&message, options)?;

		match answer {
			Options::Enable => Self::handle_enable(file),
			Options::Disable => Self::handle_disable(file),
			Options::Skip => Self::handle_skip(),
		}
	}

	fn handle_enable(file: &mut GameInfoFile) -> Result<()> {
		let search_paths = ModsConfig::MODDED_SEARCH_PATHS;
		let (start, end) = GameInfoFile::find_search_paths(file)?;

		GameInfoFile::replace_search_paths(file, &search_paths, &start, &end)?;

		Ok(())
	}

	fn handle_disable(file: &mut GameInfoFile) -> Result<()> {
		let search_paths = ModsConfig::VANILLA_SEARCH_PATHS;
		let (start, end) = GameInfoFile::find_search_paths(file)?;

		GameInfoFile::replace_search_paths(file, &search_paths, &start, &end)?;

		Ok(())
	}

	fn handle_skip() -> Result<()> {
		Ok(())
	}
}
