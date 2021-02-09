//This code prioritizes readability over length.
//I intend to make this easy to maintain and add features to in the future.

use clap::{App, AppSettings, Arg, crate_authors, crate_version};

fn main() {
	let matches = App::new("asm")
		.version(crate_version!())
		.author(crate_authors!())
		.about("A mod manager for Factorio, written in Rust!")
		.setting(AppSettings::ArgRequiredElseHelp)


		.arg(
			Arg::new("verbose")
				.about("Print every action the program takes as it is performed.")
				.short('v')
				.long("verbose")
		)
		.subcommand(
			App::new("add")
				.about("Add a mod to the current modpack")
				.aliases(&["i", "install", "add"])
		)

		.subcommand(
			App::new("remove")
				.about("Remove a mod from the current modpack")
				.aliases(&["u", "uninstall", "remove"])
		)

		.subcommand(
			App::new("start")
			.about("Start factorio with the current modpack")
			.aliases(&["s", "start", "launch"])

		)
		.subcommand(
			App::new("config")
			.about("Configure Assembler")
			.aliases(&["config", "c", "configure"])
		)


		.get_matches();


}