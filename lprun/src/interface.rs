use clap;

use platform_lp::Platform;
use version_lp::Version;
use lpsettings;
use love;

use std::path::{PathBuf,Path};

use failure::Error;

use core;
use repo;

pub fn process(matches : &clap::ArgMatches) -> Result<(),Error> {
    //! the main process function for ***run***
    //!
    //! processess all the switches and subcommands of ***run**
    //! 
    //! currently supported actions
    //! - runs a version of love
    //! - can use the `-p` or `--platform` switch to force a certain platform
    //! - can use the `-v` or `--version` switch to force a certain version

    //! process install command, if used.
    if let Some(install) = matches.subcommand_matches("install") {
        return process_install(&install);
    }

    // gets the project path, checks for the variable project.game-folder if the actual game is located
    // somewhere else
    let package_path : Option<PathBuf> = if let Some(value) = lpsettings::get_value("project.game-folder")? {
        if let Some(mut path) = get_path(&matches) {
            path.push(value.to_string());
            Some(path) // sets package_path to this
        } else {
            let mut temp_path = PathBuf::from(".");
            temp_path.push(value.to_string());
            Some(temp_path) // sets package_path to this
        }
    } else { get_path(&matches) };

    // gets the execution platform
    let plat : Platform = get_platform(&matches);

    // gets the exectuion version
    let ver : Option<Version> = get_version(&matches,&package_path);

    // runs it.
    match ver {
        None => Err(format_err!("No version found, don't know what to run.")),
        Some(ref ver) => {
            core::run(&plat,&ver,package_path)
        }
    }
}

pub fn process_install(matches : &clap::ArgMatches) -> Result<(),Error> {

    if let Some(list) = matches.subcommand_matches("list") {
        match list.is_present("list available") {
            true => repo::list_available()?,
            false => repo::list()?,
        }
    } else if let Some(_) = matches.subcommand_matches("update") {
        repo::update_local_repo(true)?;
    }


    Ok(())
}

fn get_platform(matches : &clap::ArgMatches) -> Platform {
    //! gets the platform platform to use
    //!
    //! checks if CLAP gives it a platform, if it doesn't then it goes for whatever
    //! platform the app is being run from.

    match matches.value_of("platform") {
        None => { Platform::get_user_platform() },
        Some(platform_override) => { return Platform::new(&platform_override); }
    }

}

fn get_path(matches : &clap::ArgMatches) -> Option<PathBuf> {
    match matches.value_of("PROJECT") {
        None => { return None; },
        Some(project) => {
            let path = Path::new(&project);
            return Some(path.to_path_buf());
        }
    }
}

fn get_version(matches : &clap::ArgMatches, game_path: &Option<PathBuf>) -> Option<Version> {
    //! gets the version to use

    // checks first is CLAP has a version
    if let Some(version_override) = matches.value_of("version") {
        if let Some(version) = Version::from_str(&version_override) {
            return Some(version);
        }
    }

    // checks the project for version information
    if let Some(ref path) = game_path {
        if let Ok(version_override_project) = love::project::get_required_version(path) {
            return Some(version_override_project);
        }
    }

    // if we are still here then use the latest installed version
    get_latest_installed_version()
}

fn get_latest_installed_version() -> Option<Version> { Version::from_str("0.0.0") }

pub fn app() -> clap::App<'static,'static> {
    //! [CLAP.RS](https://clap.rs/) app for easy integration.
    //!
    //! Can be easily added to any CLAP app to extend funcionality.
    //!
    //! Using ***lprun*** by itself.
    //!
    //! ```rust
    //! let app = interface::app()
    //!     .get_matches();
    //!
    //! match interface::process(&app) {
    //!     Err(error) => { println!("{}",error); }
    //!     Ok(_) => { }
    //! }
    //! ```
    //!
    //! Using ***lprun*** as part of another app.
    //!
    //! ```rust
    //! let app = clap::App("newapp")
    //!     .subcommand(interface::app().name("run"))
    //!     .get_matches();
    //!
    //! match app.subcommand() {
    //!     ("settings", Some(matches)) => { interface::process(matches); },
    //!     _ => {},
    //! }
    //! ```
    clap::App::new("lpsettings")

    // general application information
        .version(env!("CARGO_PKG_VERSION"))
        .author("snsvrno<snsvrno@tuta.io>")
        .about("Runs projects with different versions of LÖVE.")
        .name("lprun")

    // subcommands
        .subcommand(clap::SubCommand::with_name("install")
                .about("Installs different versions of LÖVE")
                .subcommand(clap::SubCommand::with_name("list")
                    .about("Lists installed binaries.")
                    .arg(clap::Arg::with_name("list available")
                        .short("a")
                        .long("list-available")
                        .help("Lists available binaries.")))
                .subcommand(clap::SubCommand::with_name("update")
                    .about("Updates the local repository of LOVE releases.")))

    // parameters
        .arg(clap::Arg::with_name("version")
            .short("v")
            .long("version")
            .help("Version of LÖVE to use, overrides PROJECT defined version.")
            .value_name("version"))

        .arg(clap::Arg::with_name("platform")
            .short("p")
            .long("platform")
            .help("Override what platform to use, can only choose 32bit varients on 64 bit machines.")
            .value_name("platform"))

    // arguements
        .arg(clap::Arg::with_name("PROJECT")
            .help("Path to LÖVE project folder or .love file")
            .value_name("PROJECT")
            .index(1))


}