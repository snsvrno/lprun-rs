extern crate clap;

extern crate love;
extern crate lpsettings;
extern crate platform_lp;
extern crate version_lp;
extern crate archive_lp;
extern crate download_lp;

#[macro_use] extern crate log;
#[macro_use] extern crate failure;

pub mod interface;

mod core;
pub use core::run;

mod binary;
mod repo;