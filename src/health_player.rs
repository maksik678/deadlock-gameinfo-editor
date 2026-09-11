use anyhow::{ Result };

use crate::cli::CommandLineInterface;
use crate::file::GameInfoFile;

const QUESTION: &str = "New player health bar style";
const TARGET: &str = "ConVars";
const KEY: &str = "citadel_unit_status_use_new";
const VALUE: &str = "true";
const KEY_2: &str = "citadel_healthbars_enabled";
const VALUE_2: &str = "false";

pub struct PlayerHealthManager;

impl PlayerHealthManager {
	pub fn process(file: &mut GameInfoFile) -> Result<()> {
		let options = vec!["Enable", "Disable", "Skip"];
		let answer = CommandLineInterface::select_prompt(&QUESTION, options)?;

		match answer {
			"Enable" => Self::handle_enable(file),
			"Disable" => Self::handle_disable(file),
			_ => Ok(()),
		}
	}

	fn handle_enable(file: &mut GameInfoFile) -> Result<()> {
		let new_line = GameInfoFile::new_line(&KEY, &VALUE);
		let new_line_2 = GameInfoFile::new_line(&KEY_2, &VALUE_2);

		match GameInfoFile::find_line(file, &KEY) {
			Ok(existing_line) => GameInfoFile::replace_line(file, &existing_line, &new_line),
			Err(_) => GameInfoFile::add_line(file, &TARGET, &new_line),
		}?;

		match GameInfoFile::find_line(file, &KEY_2) {
			Ok(existing_line) => GameInfoFile::replace_line(file, &existing_line, &new_line_2),
			Err(_) => GameInfoFile::add_line(file, &TARGET, &new_line_2),
		}?;

		Ok(())
	}

	fn handle_disable(file: &mut GameInfoFile) -> Result<()> {
		let (start, end) = GameInfoFile::find_line(file, &KEY)?;
		let (start_2, end_2) = GameInfoFile::find_line(file, &KEY_2)?;

		GameInfoFile::remove_line(file, &start, &end)?;
		GameInfoFile::remove_line(file, &start_2, &end_2)?;

		Ok(())
	}
}
