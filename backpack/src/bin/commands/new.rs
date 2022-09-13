use anyhow::Result as AnyResult;
use backpack::{
    data::{CopyMode, Opts},
    run::Runner,
};
use clap::{Arg, ArgMatches, Command};

pub fn command() -> Command<'static> {
    Command::new("new")
        .visible_alias("n")
        .about("initialize a new project")
        .arg(
            Arg::new("git")
                .short('g')
                .long("git")
                .help("clone with git")
                .takes_value(false),
        )
        .arg(
            Arg::new("overwrite")
                .short('w')
                .long("overwrite")
                .help("always overwrite target file(s)")
                .takes_value(false),
        )
        .arg(
            Arg::new("no-cache")
                .short('n')
                .long("no-cache")
                .help("fetch resources without using the cache")
                .takes_value(false),
        )
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .help("use a specified configuration file")
                .takes_value(true),
        )
        .arg(Arg::new("shortlink"))
        .arg(Arg::new("name"))
}

///
/// new will only create a new folder with contents, no overwriting and no
/// default dest folder
///
pub fn run(_matches: &ArgMatches, subcommand_matches: &ArgMatches) -> AnyResult<bool> {
    let shortlink = subcommand_matches.get_one::<String>("shortlink");
    let dest = subcommand_matches.get_one::<String>("name");
    let config_file = subcommand_matches
        .get_one::<String>("config")
        .map(String::to_string);

    let r = Runner::default();
    r.run(
        shortlink.map(String::as_str),
        dest.map(String::as_str),
        &Opts {
            show_progress: true,
            overwrite: false,
            is_git: subcommand_matches.is_present("git"),
            no_cache: subcommand_matches.is_present("no-cache"),
            always_yes: false,
            config_file,
            mode: CopyMode::Copy,
        },
    )?;

    Ok(true)
}
