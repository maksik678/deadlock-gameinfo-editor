use crate::shared::configs::GameInfoFileConfig;

use std::fs;

use anyhow::{ Context, Result };

pub struct GameInfoFile {
	path: String,
	content: String,
}

impl GameInfoFile {
	pub fn init() -> Result<GameInfoFile> {
		let path = Self::find()?;
		let content = Self::read(&path)?;

		Ok(Self { path, content })
	}

	pub fn save(&self) -> Result<()> {
		let result = fs::write(&self.path, &self.content)?;

		Ok(result)
	}

	fn find() -> Result<String> {
		let steam_dir = steamlocate::locate()?;
		let steam_app = steam_dir.find_app(GameInfoFileConfig::DEADLOCK_APP_ID)?;

		let (deadlock, library) = &steam_app.context("[Deadlock App] was not found")?;

		let deadlock_path = &library.resolve_app_dir(&deadlock);
		let gameinfo_path = &deadlock_path.join("game").join("citadel").join("gameinfo.gi");
		let string_path = gameinfo_path.to_string_lossy().to_string();

		Ok(string_path)
	}

	fn read(path: &String) -> Result<String> {
		let file = fs::read_to_string(path)?;

		Ok(file)
	}

	pub fn find_convars(&self) -> Result<usize> {
		let target_pos = self.content.find("ConVars").context("[ConVars] was not found")?;
		let open_brace_pos = self.content[target_pos..].find('{').context("[Open Brace] was not found")?;
		let convars_pos = target_pos + open_brace_pos + 1;

		Ok(convars_pos)
	}

	pub fn find_section(&self, section: &(&str, &str)) -> Result<(usize, usize)> {
		let (section_start, section_end) = section;

		let section_start_pos = self.content.find(section_start).context("[{section_start}] was not found")? - 3;
		let section_end_pos = self.content.find(section_end).context("[{section_end}] was not found")? + section_end.len() + 1;

		Ok((section_start_pos, section_end_pos))
	}

	pub fn add_section(&mut self, lines: &Vec<(&str, &str)>, section: &(&str, &str), convars_pos: &usize) -> Result<()> {
		let content = Self::create_content(&lines);
		let section_content = Self::create_section_content(&content, section);

		self.content.replace_range(convars_pos..convars_pos, &section_content);

		Ok(())
	}

	pub fn replace_section(&mut self, lines: &Vec<(&str, &str)>, section: &(&str, &str), section_pos: &(usize, usize)) -> Result<()> {
		let (section_start_pos, section_end_pos) = section_pos;

		let content = Self::create_content(&lines);
		let section_content = Self::create_section_content(&content, section);

		self.content.replace_range(section_start_pos..section_end_pos, &section_content);

		Ok(())
	}

	pub fn remove_section(&mut self, section_pos: &(usize, usize)) -> Result<()> {
		let (section_start_pos, section_end_pos) = section_pos;

		self.content.replace_range(section_start_pos..section_end_pos, "");

		Ok(())
	}

	fn create_content(lines: &Vec<(&str, &str)>) -> String {
		let mut content = String::from("");

		for kv in lines {
			let (key, value) = kv;
			let line = String::from(format!("\n\t\t\"{key}\"\t\"{value}\""));
			content.push_str(&line);
		}

		content
	}

	fn create_section_content(content: &String, section: &(&str, &str)) -> String {
		let (section_start, section_end) = section;
		let section_content = format!("\n\t\t{section_start}{content}\n\t\t{section_end}\n");

		section_content
	}
}
