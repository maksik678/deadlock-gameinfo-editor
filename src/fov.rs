use anyhow::{ Result };

use crate::cli::CommandLineInterface;
use crate::file::GameInfoFile;

const QUESTION: &str = "Support for additional FOV";
const CHOOSE: &str = "Select additional FOV";
const TARGET: &str = "ConVars";
const KEY: &str = "r_aspectratio";

pub struct FovManager;

impl FovManager {
	pub fn process(file: &String) -> Result<String> {
		let options = vec!["Enable", "Disable", "Skip"];

		let answer = CommandLineInterface::select_prompt(&QUESTION, options)?;

		match answer {
			"Enable" => Self::handle_enable(file),
			"Disable" => Self::handle_disable(file),
			_ => Self::handle_skip(file),
		}
	}

	fn handle_enable(file: &String) -> Result<String> {
		let options = vec!["2.1", "2.2", "2.3", "2.4", "2.5", "2.6", "2.7", "2.8", "2.9", "3.0"];

		let value = CommandLineInterface::select_prompt(&CHOOSE, options)?;
		let new_line = GameInfoFile::new_line(&KEY, &value);

		match GameInfoFile::find_line(file, &KEY) {
			Ok(existing) => GameInfoFile::replace_line(file, &existing, &new_line),
			Err(_) => GameInfoFile::add_line(file, &TARGET, &new_line),
		}
	}

	fn handle_disable(file: &String) -> Result<String> {
		let (start, end) = GameInfoFile::find_line(file, &KEY)?;
		GameInfoFile::remove_line(file, &start, &end)
	}

	fn handle_skip(file: &String) -> Result<String> {
		Ok(file.clone())
	}
}
