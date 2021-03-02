//This code prioritizes readability over length.
//I intend to make this easy to maintain and add features to in the future.

//import crates
use clap::{App, AppSettings, Arg, crate_authors, crate_version};
use std::env;
use ini::Ini;


pub fn main() {
    let _matches = App::new("asm")
        .version(crate_version!())
        .author(crate_authors!())
        .about("A mod manager for Factorio, written in Rust!")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColorAuto)
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::StrictUtf8)
        .setting(AppSettings::VersionlessSubcommands)

        .arg(
            Arg::new("verbose")
                .about("Print every action the program takes as it is performed.")
                .short('v')
                .long("verbose")
                .multiple(false)
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

                .arg(
                    Arg::new("edit")
                        .about("Open config file in your editor")
                        .short('e')
                        .long("edit")
                        .takes_value(false)
                        .exclusive(true)

                )
        )
        .subcommand(
            App::new("factorio")
            .about("Subcommands related to Factorio installations")
               .aliases(&["factorio", "fac", "f"])
            .setting(AppSettings::VersionlessSubcommands)

            .subcommand(
                App::new("install")
                    .about("Install a version of Factorio")
                    .aliases(&["install", "i", "add"])


            )
                .subcommand(
                    App::new("uninstall")
                        .about("Uninstall a version of Factorio")
                        .aliases(&["uninstall", "u", "remove"])
                )
                .subcommand(
                    App::new("asdf")
                )
        )


        .get_matches();

}