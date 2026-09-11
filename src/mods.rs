use anyhow::Result;

use crate::cli::CommandLineInterface;
use crate::file::GameInfoFile;

const MODDED_SEARCH_PATHS: &str =
	"SearchPaths
        {  
            Game_Language       citadel_*LANGUAGE*
            Game                citadel/addons
            Mod                 citadel
            Write               citadel          
            Game                citadel
            Mod                 core
            Write               core
            Game                core        
        }";

const VANILLA_SEARCH_PATHS: &str =
	"SearchPaths
        {  
            Game_Language       citadel_*LANGUAGE*
            Game                citadel
            Write               citadel          
            Game                citadel
            Write               core
            Game                core        
        }";

const QUESTION: &str = "Support for Mods";

pub struct ModsManager;

impl ModsManager {
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
		let (start, end) = GameInfoFile::find_search_paths(file)?;
		GameInfoFile::replace_search_paths(file, &MODDED_SEARCH_PATHS, &start, &end)
	}

	fn handle_disable(file: &String) -> Result<String> {
		let (start, end) = GameInfoFile::find_search_paths(file)?;
		GameInfoFile::replace_search_paths(file, &VANILLA_SEARCH_PATHS, &start, &end)
	}

	fn handle_skip(file: &String) -> Result<String> {
		Ok(file.clone())
	}
}
