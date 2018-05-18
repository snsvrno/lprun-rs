use clap;

use platform::Platform;
use version::version::Version;
use lpsettings;
use love::project::project;

use std::path::{PathBuf,Path};

use core;

pub fn process(matches : &clap::ArgMatches) -> Result<(),&'static str> {
  //! the main process function for ***run***
  //!
  //! processess all the switches and subcommands of ***run**
  //! 
  //! currently supported actions
  //! - runs a version of love
  //! - can use the `-p` or `--platform` switch to force a certain platform
  //! - can use the `-v` or `--version` switch to force a certain version

  // gets the project path, checks for the variable project.game-folder if the actual game is located
  // somewhere else
  let mut package_path : Option<PathBuf> = if let Some(value) = lpsettings::get_value("project.game-folder") {
    if let Some(mut path) = get_path(&matches) {
      path.push(value);
      Some(path) // sets package_path to this
    } else {
      let mut temp_path = PathBuf::from(".");
      temp_path.push(value);
      Some(temp_path) // sets package_path to this
    }
  } else { get_path(&matches) };
  let plat : Platform = get_platform(&matches);
  let ver : Option<Version> = get_version(&matches,&package_path);

  if let Some(ref ver) = ver { return core::run(&plat,&ver,package_path); }
  else { return Err("No version to use found"); }
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
  match matches.value_of("version") {
    None => { },
    Some(version_override) => {
      match Version::from_str(&version_override) {
        None => { },
        Some(ver) => { return Some(ver); }
      }
    }
  }

  // checks the project for version information
  match game_path {
    &Some(ref path) => {
      match project::get_required_version(&path) {
        Err( _ ) => { return None; }
        Ok(version_override_project) => { return Some(version_override_project); }
      }
    }
    _ => { None }
  }
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
  //!   .get_matches();
  //!
  //! match interface::process(&app) {
  //!   Err(error) => { println!("{}",error); }
  //!   Ok(_) => { }
  //! }
  //! ```
  //!
  //! Using ***lprun*** as part of another app.
  //!
  //! ```rust
  //! let app = clap::App("newapp")
  //!   .subcommand(interface::app().name("run"))
  //!   .get_matches();
  //!
  //! match app.subcommand() {
  //!   ("settings", Some(matches)) => { interface::process(matches); },
  //!   _ => {},
  //! }
  //! ```
  clap::App::new("lpsettings")

  // general application information
    .version(env!("CARGO_PKG_VERSION"))
    .author("snsvrno<snsvrno@tuta.io>")
    .about("Runs projects with different versions of LÖVE.")
    .name("lprun")

  // switches

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