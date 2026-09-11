use crate::core::cli::CommandLineInterface;
use crate::core::file::GameInfoFile;
use crate::shared::configs::OptimizationsConfig;
use crate::shared::enums::{ Options, PresetOptions };
use crate::shared::traits::Manager;

use anyhow::Result;
use strum::IntoEnumIterator;

pub struct OptimizationsManager;

impl Manager for OptimizationsManager {
	fn process(file: &mut GameInfoFile) -> Result<()> {
		let message = OptimizationsConfig::PROMPT_MESSAGE;
		let options = Options::iter().collect();

		let answer = CommandLineInterface::select_prompt(&message, options)?;

		match answer {
			Options::Enable => Self::handle_enable(file),
			Options::Disable => Self::handle_disable(file),
			Options::Skip => Self::handle_skip(),
		}
	}

	fn handle_enable(file: &mut GameInfoFile) -> Result<()> {
		let section = (OptimizationsConfig::SECTION_START, OptimizationsConfig::SECTION_END);

		let convars_pos = GameInfoFile::find_convars(file)?;

		let options = PresetOptions::iter().collect();
		let message = OptimizationsConfig::SELECT_MESSAGE;
		let answer = CommandLineInterface::select_prompt(&message, options)?;

		let lines = match answer {
			PresetOptions::Light => OptimizationsConfig::LIGHT_LINES.to_vec(),
			PresetOptions::Medium => OptimizationsConfig::MEDIUM_LINES.to_vec(),
			PresetOptions::Potato => OptimizationsConfig::POTATO_LINES.to_vec(),
		};

		match GameInfoFile::find_section(file, &section) {
			Ok(section_pos) => GameInfoFile::replace_section(file, &lines, &section, &section_pos),
			Err(..) => GameInfoFile::add_section(file, &lines, &section, &convars_pos),
		}
	}

	fn handle_disable(file: &mut GameInfoFile) -> Result<()> {
		let section = (OptimizationsConfig::SECTION_START, OptimizationsConfig::SECTION_END);

		match GameInfoFile::find_section(file, &section) {
			Ok(section_pos) => GameInfoFile::remove_section(file, &section_pos),
			Err(..) => Ok(()),
		}
	}
}
