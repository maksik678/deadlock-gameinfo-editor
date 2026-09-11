use anyhow::{ Result };

use crate::cli::CommandLineInterface;
use crate::file::GameInfoFile;

const QUESTION: &str = "Show minion health through walls & objects";
const TARGET: &str = "ConVars";
const KEY: &str = "citadel_damage_offscreen_indicator_disabled";
const VALUE: &str = "0";

pub struct MinionHealthManager;

impl MinionHealthManager {
	pub fn process(file: &String) -> Result<String> {
		let options = vec!["Enable", "Disable", "Skip"];

		let answer = CommandLineInterface::select_prompt(&QUESTION, options)?;

		match answer {
			"Enable" => Self::handle_enable(&file),
			"Disable" => Self::handle_disable(&file),
			_ => Self::handle_skip(&file),
		}
	}

	fn handle_enable(file: &String) -> Result<String> {
		let new_line = GameInfoFile::new_line(&KEY, &VALUE);

		let file = (match GameInfoFile::find_line(&file, &KEY) {
			Ok(existing) => GameInfoFile::replace_line(&file, &existing, &new_line),
			Err(_) => GameInfoFile::add_line(&file, &TARGET, &new_line),
		})?;

		Ok(file.clone())
	}

	fn handle_disable(file: &String) -> Result<String> {
		let (start, end) = GameInfoFile::find_line(&file, &KEY)?;
		let file = GameInfoFile::remove_line(&file, &start, &end)?;

		Ok(file.clone())
	}

	fn handle_skip(file: &String) -> Result<String> {
		Ok(file.clone())
	}
}
