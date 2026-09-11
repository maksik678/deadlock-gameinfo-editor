use anyhow::{ Result };

use crate::cli::CommandLineInterface;
use crate::file::GameInfoFile;

const QUESTION: &str = "Show minion health through walls & objects";
const TARGET: &str = "ConVars";
const KEY: &str = "citadel_damage_offscreen_indicator_disabled";
const VALUE: &str = "0";

pub struct MinionHealthManager;

impl MinionHealthManager {
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

		match GameInfoFile::find_line(file, &KEY) {
			Ok(existing_line) => GameInfoFile::replace_line(file, &existing_line, &new_line),
			Err(_) => GameInfoFile::add_line(file, &TARGET, &new_line),
		}?;

		Ok(())
	}

	fn handle_disable(file: &mut GameInfoFile) -> Result<()> {
		let (start, end) = GameInfoFile::find_line(file, &KEY)?;
		GameInfoFile::remove_line(file, &start, &end)?;

		Ok(())
	}
}
