use crate::core::cli::CommandLineInterface;
use crate::core::file::GameInfoFile;
use crate::shared::configs::MinionHealthConfig;
use crate::shared::enums::Options;
use crate::shared::traits::Manager;

use anyhow::Result;
use strum::IntoEnumIterator;

pub struct MinionHealthManager;

impl Manager for MinionHealthManager {
	fn process(file: &mut GameInfoFile) -> Result<()> {
		let message = MinionHealthConfig::PROMPT_MESSAGE;
		let options = Options::iter().collect();

		let answer = CommandLineInterface::select_prompt(&message, options)?;

		match answer {
			Options::Enable => Self::handle_enable(file),
			Options::Disable => Self::handle_disable(file),
			Options::Skip => Self::handle_skip(),
		}
	}

	fn handle_enable(file: &mut GameInfoFile) -> Result<()> {
		let section = (MinionHealthConfig::SECTION_START, MinionHealthConfig::SECTION_END);
		let lines = MinionHealthConfig::LINES.to_vec();

		let convars_pos = GameInfoFile::find_convars(file)?;

		match GameInfoFile::find_section(file, &section) {
			Ok(section_pos) => GameInfoFile::replace_section(file, &lines, &section, &section_pos),
			Err(..) => GameInfoFile::add_section(file, &lines, &section, &convars_pos),
		}
	}

	fn handle_disable(file: &mut GameInfoFile) -> Result<()> {
		let section = (MinionHealthConfig::SECTION_START, MinionHealthConfig::SECTION_END);

		match GameInfoFile::find_section(file, &section) {
			Ok(section_pos) => GameInfoFile::remove_section(file, &section_pos),
			Err(..) => Ok(()),
		}
	}
}
