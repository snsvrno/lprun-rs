use platform_lp::Platform;
use version_lp::Version;

use failure::Error;

use std::path::{Path,PathBuf};
use binary;

pub fn run<P : AsRef<Path>>(plat : &Platform, ver : &Version, package_path : Option<P>) -> Result<(),Error> {
    //! runs love based on a ***platform*** and a ***version***

    let exe_path = PathBuf::from(binary::build_path(plat,ver)?);
    if !exe_path.exists() {
        info!("love {} {} not found, attempting to install.",plat,ver);
        binary::install(plat,ver)?;
    }

    let package = if let Some(path) = package_path {
        let path = PathBuf::from(path.as_ref());
        info!("Found love project at '{}'",path.display().to_string());
        Some(path)
    } else {
        None
    };

    binary::run(exe_path,package)
}