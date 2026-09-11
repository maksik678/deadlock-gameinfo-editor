use crate::core::cli::CommandLineInterface;
use crate::core::file::GameInfoFile;
use crate::shared::configs::PlayerHealthConfig;
use crate::shared::enums::Options;

use anyhow::Result;
use strum::IntoEnumIterator;

pub struct PlayerHealthManager;

impl PlayerHealthManager {
	pub fn process(file: &mut GameInfoFile) -> Result<()> {
		let message = PlayerHealthConfig::PROMPT_MESSAGE;
		let options = Options::iter().collect();

		let answer = CommandLineInterface::select_prompt(&message, options)?;

		match answer {
			Options::Enable => Self::handle_enable(file),
			Options::Disable => Self::handle_disable(file),
			Options::Skip => Self::handle_skip(),
		}
	}

	fn handle_enable(file: &mut GameInfoFile) -> Result<()> {
		let target = PlayerHealthConfig::TARGET;

		let key = PlayerHealthConfig::KEY;
		let value = PlayerHealthConfig::VALUE;
		let new_line = GameInfoFile::create_line(&key, &value);

		match GameInfoFile::find_line(file, &key) {
			Ok(existing_line) => GameInfoFile::replace_line(file, &existing_line, &new_line),
			Err(_) => GameInfoFile::add_line(file, &target, &new_line),
		}?;

		let key_2 = PlayerHealthConfig::KEY_2;
		let value_2 = PlayerHealthConfig::VALUE_2;
		let new_line_2 = GameInfoFile::create_line(&key_2, &value_2);

		match GameInfoFile::find_line(file, &key_2) {
			Ok(existing_line) => GameInfoFile::replace_line(file, &existing_line, &new_line_2),
			Err(_) => GameInfoFile::add_line(file, &target, &new_line_2),
		}?;

		Ok(())
	}

	fn handle_disable(file: &mut GameInfoFile) -> Result<()> {
		let key = PlayerHealthConfig::KEY;
		let key_2 = PlayerHealthConfig::KEY_2;
		let (start, end) = GameInfoFile::find_line(file, &key)?;
		let (start_2, end_2) = GameInfoFile::find_line(file, &key_2)?;

		GameInfoFile::remove_line(file, &start, &end)?;
		GameInfoFile::remove_line(file, &start_2, &end_2)?;

		Ok(())
	}

	fn handle_skip() -> Result<()> {
		Ok(())
	}
}
