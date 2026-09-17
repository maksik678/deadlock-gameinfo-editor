mod core {
	pub mod cli;
	pub mod file;
}
mod managers {
	pub mod fov;
	pub mod health_minion;
	pub mod health_player;
	pub mod optimizations;
}

mod shared {
	pub mod configs;
	pub mod enums;
	pub mod traits;
}

use crate::core::cli::CommandLineInterface;

use anyhow::Result;

fn main() -> Result<()> {
	if let Err(e) = CommandLineInterface::run() {
		CommandLineInterface::error(e);
		CommandLineInterface::exit()
	} else {
		CommandLineInterface::exit()
	}
}
