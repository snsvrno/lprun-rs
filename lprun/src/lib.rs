/// lovepack library for managing different distributions of LOVE

// for interfacing
#[cfg(feature = "cli")]
extern crate clap;

// lovepack tooling
extern crate love;
extern crate lpsettings;
extern crate platform_lp;
extern crate version_lp;
extern crate archive_lp;
extern crate download_lp;

// for retrieveing release information and saving it
extern crate reqwest;
extern crate serde_json;
extern crate regex;
extern crate toml;
#[macro_use] extern crate serde_derive;
extern crate serde;

#[cfg(feature = "cli")]
extern crate prettytable;

#[macro_use] extern crate smart_hash;
#[macro_use] extern crate smart_hash_derive;

// for creating good functions
#[macro_use] extern crate log;
#[macro_use] extern crate failure;

// the public interface for CLI apps (if feature is enabled)
#[cfg(feature = "cli")]
pub mod interface;

mod core;
mod binary;
mod repo;
mod structs;

// the public interface for the library
pub use core::run as run;
pub use binary::install as install;