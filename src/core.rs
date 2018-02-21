use clap;

//use shared::{version,platform,paths};
//use structures::bin::*;

use platform::Platform;
use version::version::Version;
use love::binary::binary::Binary;

use std::path::PathBuf;

pub fn run (plat : &Platform, ver : &Version, package_path : Option<PathBuf>) -> Result<(),&'static str> {
  //! runs love based on a ***platform*** and a ***version***
  let mut binary = Binary::new(plat,&ver,None);
  binary.run();
  Ok(())
}

fn get_bin_version_path(plat : &Platform, ver : &Version) -> Result<PathBuf,&'static str> {
  Err("No binary found")
}