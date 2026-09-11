use anyhow::{ anyhow, Result };

const DEADLOCK_APP_ID: u32 = 1422450;

const NEW_LINE: &str = "\n";
const INDENT: &str = "\t\t";
const SEPARATOR: &str = "\t";

pub struct GameInfoFile {}

impl GameInfoFile {
	pub fn init() -> Result<(String, String)> {
		let path = Self::find()?;
		let file = Self::read(&path)?;

		Ok((path, file))
	}

	fn find() -> Result<String> {
		let steam_dir = steamlocate::locate()?;
		let steam_app = steam_dir.find_app(DEADLOCK_APP_ID)?;

		let (deadlock, library) = &steam_app.ok_or_else(|| anyhow!("Deadlock not found"))?;

		let deadlock_path = &library.resolve_app_dir(&deadlock);
		let gameinfo_path = &deadlock_path.join("game").join("citadel").join("gameinfo.gi");
		let string_path = gameinfo_path.to_string_lossy().to_string();

		Ok(string_path)
	}

	fn read(path: &String) -> Result<String> {
		let file = std::fs::read_to_string(path)?;

		Ok(file)
	}

	pub fn save(path: &String, content: &String) -> Result<()> {
		let result = std::fs::write(path, content)?;

		Ok(result)
	}

	pub fn new_line(key: &str, value: &str) -> String {
		let line = format!("{NEW_LINE}{INDENT}\"{key}\"{SEPARATOR}\"{value}\"");

		line
	}

	pub fn find_line(file: &String, key: &str) -> Result<(usize, usize)> {
		let content = file.clone();

		let key_pos = content.find(key).ok_or_else(|| anyhow!("Key — {key} not found"))?;

		let line_start = content[..key_pos]
			.rfind('\n')
			.map(|pos| pos + 1)
			.unwrap_or(0);

		let line_end = content[key_pos..]
			.find('\n')
			.map(|pos| key_pos + pos + 1)
			.unwrap_or(content.len());

		Ok((line_start, line_end))
	}

	pub fn add_line(file: &String, target: &str, new_line: &str) -> Result<String> {
		let mut content = file.clone();

		let target_pos = content.find(target).ok_or_else(|| anyhow!("{target} section not found"))?;
		let open_brace_pos = content[target_pos..].find('{').ok_or_else(|| anyhow!("{target} start of section not found"))?;

		let insert_pos = target_pos + open_brace_pos + 1;
		content.replace_range(insert_pos..insert_pos, &new_line);

		Ok(content)
	}

	pub fn replace_line(file: &String, old_line: &(usize, usize), new_line: &str) -> Result<String> {
		let mut content = file.clone();

		let (line_start, line_end) = old_line;
		let line_start = line_start - 1;
		let line_end = *line_end;

		let new_line = format!("{new_line}\n");

		content.replace_range(line_start..line_end, &new_line);

		Ok(content)
	}

	pub fn remove_line(file: &String, line_start: &usize, line_end: &usize) -> Result<String> {
		let mut content = file.clone();

		content.replace_range(line_start..line_end, "");

		Ok(content)
	}

	pub fn find_search_paths(file: &String) -> Result<(usize, usize)> {
		let content = file.clone();

		let search_paths_start = content.find("SearchPaths").ok_or_else(|| anyhow!("SearchPaths section not found"))?;
		let relative_end = content[search_paths_start..].find('}').ok_or_else(|| anyhow!("SearchPaths end of section not found"))?;
		let search_paths_end = search_paths_start + relative_end + 1;

		Ok((search_paths_start, search_paths_end))
	}

	pub fn replace_search_paths(file: &String, search_paths: &str, search_paths_start: &usize, search_paths_end: &usize) -> Result<String> {
		let mut content = file.clone();

		content.replace_range(search_paths_start..search_paths_end, &search_paths);

		Ok(content)
	}
}
