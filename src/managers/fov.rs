use crate::core::cli::CommandLineInterface;
use crate::core::file::GameInfoFile;
use crate::shared::configs::FovConfig;
use crate::shared::enums::Options;

use anyhow::Result;
use strum::IntoEnumIterator;

pub struct FovManager;

impl FovManager {
	pub fn process(file: &mut GameInfoFile) -> Result<()> {
		let message = FovConfig::PROMPT_MESSAGE;
		let options = Options::iter().collect();

		let answer = CommandLineInterface::select_prompt(&message, options)?;

		match answer {
			Options::Enable => Self::handle_enable(file),
			Options::Disable => Self::handle_disable(file),
			Options::Skip => Self::handle_skip(),
		}
	}

	fn handle_enable(file: &mut GameInfoFile) -> Result<()> {
		let message = FovConfig::SELECT_MESSAGE;
		let options = FovConfig::SELECT_OPTIONS.to_vec();

		let key = FovConfig::KEY;
		let value = CommandLineInterface::select_prompt(&message, options)?;

		let target = FovConfig::TARGET;
		let new_line = GameInfoFile::create_line(&key, &value);

		match GameInfoFile::find_line(file, &key) {
			Ok(existing_line) => GameInfoFile::replace_line(file, &existing_line, &new_line),
			Err(_) => GameInfoFile::add_line(file, &target, &new_line),
		}?;

		Ok(())
	}

	fn handle_disable(file: &mut GameInfoFile) -> Result<()> {
		let key = FovConfig::KEY;
		let (start, end) = GameInfoFile::find_line(file, &key)?;

		GameInfoFile::remove_line(file, &start, &end)?;

		Ok(())
	}

	fn handle_skip() -> Result<()> {
		Ok(())
	}
}
