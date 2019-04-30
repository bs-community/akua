use clap::{crate_authors, crate_version, App, AppSettings, Arg, SubCommand};

mod install;
mod switch;

fn main() {
    let matches = App::new("アクア")
        .version(crate_version!())
        .version_short("-v")
        .author(crate_authors!())
        .setting(AppSettings::DisableHelpSubcommand)
        .subcommand(SubCommand::with_name("fast").about("Switch to fast channel. (`dev` branch)"))
        .subcommand(
            SubCommand::with_name("slow").about("Switch to slow channel. (`master` branch)"),
        )
        .subcommand(
            SubCommand::with_name("stable")
                .about("Switch to stable channel with specified version.")
                .arg(Arg::with_name("version")),
        )
        .get_matches();

    let result = match matches.subcommand() {
        ("fast", _) => switch::branch("dev"),
        ("slow", _) => switch::branch("master"),
        ("stable", Some(cmd)) => {
            let version = cmd.value_of("version").expect("You must specify version.");
            switch::tag(version)
        }
        _ => switch::default(),
    };
    let result = result
        .map_err(|err| err.message().to_string())
        .and_then(|requirements| install::install(requirements).map_err(|err| err.to_string()));
    if let Err(message) = result {
        println!("{}", message);
        std::process::exit(1);
    }

    println!("Completed!");
}
