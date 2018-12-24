extern crate clap;

extern crate love;
extern crate lpsettings;
extern crate platform_lp;
extern crate version_lp;
extern crate archive_lp;
extern crate download_lp;

extern crate reqwest;
extern crate serde_json;
extern crate regex;
extern crate toml;
#[macro_use] extern crate serde_derive;
extern crate serde;

#[macro_use] extern crate prettytable;
#[macro_use] extern crate smart_hash;
#[macro_use] extern crate smart_hash_derive;

#[macro_use] extern crate log;
#[macro_use] extern crate failure;

pub mod interface;

mod core;
pub use core::run;

mod binary;
mod repo;

mod structs;