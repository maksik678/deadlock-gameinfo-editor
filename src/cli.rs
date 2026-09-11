use crossterm::{ execute, terminal::SetTitle };
use inquire::{ Select, Text };
use anyhow::Result;

const TITLE: &str = "Deadlock gameinfo.gi editor";

pub struct CommandLineInterface {}

impl CommandLineInterface {
	pub fn init() -> Result<()> {
		execute!(std::io::stdout(), SetTitle(&TITLE))?;

		Ok(())
	}

	pub fn exit() -> Result<()> {
		Text::new("Press Enter to exit...").prompt()?;

		Ok(())
	}

	pub fn select_prompt<'a>(message: &'a str, options: Vec<&'a str>) -> Result<&'a str> {
		let answer = Select::new(message, options.to_vec()).with_page_size(10).prompt()?;

		Ok(answer)
	}
}
