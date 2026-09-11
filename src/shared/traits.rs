use crate::core::file::GameInfoFile;

use anyhow::Result;

pub trait Manager {
	fn process(file: &mut GameInfoFile) -> Result<()>;
	fn handle_enable(file: &mut GameInfoFile) -> Result<()>;
	fn handle_disable(file: &mut GameInfoFile) -> Result<()>;
	fn handle_skip() -> Result<()> {
		Ok(())
	}
}
