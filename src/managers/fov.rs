use crate::core::cli::CommandLineInterface;
use crate::core::file::GameInfoFile;
use crate::shared::configs::FovConfig;
use crate::shared::enums::Options;
use crate::shared::traits::Manager;

use anyhow::Result;
use strum::IntoEnumIterator;

pub struct FovManager;

impl Manager for FovManager {
	fn process(file: &mut GameInfoFile) -> Result<()> {
		let message = FovConfig::PROMPT_MESSAGE;
		let options = Options::iter().collect();

		let answer = CommandLineInterface::select_prompt(message, options)?;

		match answer {
			Options::Enable => Self::handle_enable(file),
			Options::Disable => Self::handle_disable(file),
			Options::Skip => Self::handle_skip(),
		}
	}

	fn handle_enable(file: &mut GameInfoFile) -> Result<()> {
		let section = (FovConfig::SECTION_START, FovConfig::SECTION_END);

		let message = FovConfig::SELECT_MESSAGE;
		let options = FovConfig::VALUE_OPTIONS.to_vec();

		let key = FovConfig::KEY;
		let value = CommandLineInterface::select_prompt(message, options)?;
		let lines = vec![(key, value)];

		let convars_pos = GameInfoFile::find_convars(file)?;

		match GameInfoFile::find_section(file, section) {
			Ok(section_pos) => GameInfoFile::replace_section(file, lines, section, section_pos),
			Err(..) => GameInfoFile::add_section(file, lines, section, convars_pos),
		}
	}

	fn handle_disable(file: &mut GameInfoFile) -> Result<()> {
		let section = (FovConfig::SECTION_START, FovConfig::SECTION_END);

		match GameInfoFile::find_section(file, section) {
			Ok(section_pos) => GameInfoFile::remove_section(file, section_pos),
			Err(..) => Ok(()),
		}
	}
}
