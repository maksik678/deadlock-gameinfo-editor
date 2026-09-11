pub struct CommandLineInterfaceConfig;
impl CommandLineInterfaceConfig {
	pub const TITLE: &str = "Deadlock gameinfo.gi editor";
	pub const EXIT_MESSAGE: &str = "Press Enter to exit...";
}

pub struct GameInfoFileConfig;
impl GameInfoFileConfig {
	pub const DEADLOCK_APP_ID: u32 = 1422450;
}

pub struct FovConfig;
impl FovConfig {
	pub const PROMPT_MESSAGE: &str = "Additional FOV";
	pub const SELECT_MESSAGE: &str = "Select additional FOV";
	pub const SELECT_OPTIONS: [&str; 10] = ["2.1", "2.2", "2.3", "2.4", "2.5", "2.6", "2.7", "2.8", "2.9", "3.0"];
	pub const TARGET: &str = "ConVars";
	pub const KEY: &str = "r_aspectratio";
}

pub struct MinionHealthConfig;
impl MinionHealthConfig {
	pub const PROMPT_MESSAGE: &str = "Show minion health through walls & objects";
	pub const TARGET: &str = "ConVars";
	pub const KEY: &str = "citadel_damage_offscreen_indicator_disabled";
	pub const VALUE: &str = "0";
}

pub struct PlayerHealthConfig;
impl PlayerHealthConfig {
	pub const PROMPT_MESSAGE: &str = "New player health bar style";
	pub const TARGET: &str = "ConVars";
	pub const KEY: &str = "citadel_unit_status_use_new";
	pub const VALUE: &str = "true";
	pub const KEY_2: &str = "citadel_healthbars_enabled";
	pub const VALUE_2: &str = "false";
}

pub struct ModsConfig;
impl ModsConfig {
	pub const PROMPT_MESSAGE: &str = "Support for Mods";

	pub const MODDED_SEARCH_PATHS: &str =
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

	pub const VANILLA_SEARCH_PATHS: &str =
		"SearchPaths
        {  
            Game_Language       citadel_*LANGUAGE*
            Game                citadel
            Write               citadel          
            Game                citadel
            Write               core
            Game                core        
        }";
}
