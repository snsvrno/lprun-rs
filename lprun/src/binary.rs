use platform_lp::Platform;
use version_lp::Version;
use lpsettings;
use download_lp;
use archive_lp;

use failure::Error;

use std::path::{Path, PathBuf};
use std::process::Command;
use std::fs::{create_dir_all,remove_file,read_dir};

use std::collections::HashSet;
use structs::release::Release;
use repo;

pub fn build_path(platform : &Platform, version : &Version) -> Result<PathBuf,Error> {
    //! generates the path to the binary, used for executing.
    //! 
    //! ```rust,ignore
    //! # // ignoring this because this isn't a public interface
    //! # 
    //! # extern crate platform_lp; use platform_lp::Platform;
    //! # extern crate version_lp; use version_lp::Version;
    //! # extern crate lprun; use lprun::binary;
    //! # use std::path::PathBuf;
    //! # 
    //! # 
    //! let path = binary::build_path(Platform::Nix64, Version::new(&[0,10,2]));
    //! 
    //! # let path = Ok(PathBuf::from("~/.lovepack/bin/nix64/0.10.2/love"));
    //! 
    //! match path {
    //!     Err(_) => assert!(false),
    //!     Ok(path) => assert_eq!(path,PathBuf::from("~/.lovepack/bin/nix64/0.10.2/love"))
    //! }
    //! ```

    let mut path = lpsettings::get_folder();
    let binary_path = lpsettings::get_value_or("run.binaries-root",&"bin".to_string());

    path.push(binary_path.to_string());
    path.push(platform.to_short_string());

    if platform == &Platform::Win32 || platform == &Platform::Win64 { 
        path.push(version.to_string());
        path.push("love.exe"); 
    } else if platform == &Platform::Nix32 || platform == &Platform::Nix64 { 
        path.push(version.to_string());
        path.push("love"); 
    } 

    Ok(path)
}

pub fn install(platform : &Platform, version : &Version) -> Result<PathBuf,Error> {
    //! doesn't check if it already exists, you should do this before.
    //! 
    //! will install the desired version in the local repo stop. if the folder already exists
    //! then it assumes it is already installed and returns that path.
    
    let install_exe = build_path(platform,version)?;
    match install_exe.parent() {
        None => Err(format_err!("Couldn't get the folder for the path to install: {}",
            install_exe.display().to_string())),
        Some(install_path) => {
            if !install_path.exists() {
                create_dir_all(install_path)?;

                let link = repo::get_version_link(platform,version)?;
                info!("Installing from '{}'",link);
                let (download_file_name,_size) = download_lp::download(&link, install_path.display().to_string())?;
                let download_path = {
                    let mut path = PathBuf::from(install_path);
                    path.push(download_file_name);
                    path
                };
                let exe_path = archive_lp::extract_root_to(&download_path.display().to_string(), &install_path.display().to_string())?;

                remove_file(download_path)?;

                Ok(exe_path)
            } else {
                info!("Path '{}' already exists, assuming it was already installed.",
                    install_path.display().to_string());
                Ok(install_exe.clone())
            }
        }
    }
}

pub fn run<P:AsRef<Path>>(binary_path : P, package_path : Option<PathBuf>) -> Result<(),Error> {
    //! doesn't check if it exists, you should check before using this.
    //! 
    //! direct run function. will try and run the app and error if it can't it will not
    //! attempt to install the binary, use core::run instead if you want that functionality.
    
    let path = PathBuf::from(binary_path.as_ref());
    
    let mut command = Command::new(&path);
    if let Some(package_path) = package_path { command.arg(package_path); }

    match command.spawn() {
        Err(error) => Err(format_err!("{}",error)),
        Ok(_child) => Ok(()) // maybe use this in the future?
    }
}

#[cfg(feature = "cli")]
pub fn get_installed() -> Result<HashSet<Release>,Error> {
    //! returns a HashSet of all installed releases.
    //! 
    //! primarily for cli output

    let mut releases : HashSet<Release> = HashSet::new();
    
    let base_path = {
        let mut path = lpsettings::get_folder();
        let binary_path = lpsettings::get_value_or("run.binaries-root",&"bin".to_string());
        path.push(binary_path.to_string());
        path
    };

    for entry in read_dir(base_path)? {
        let entry = entry?;
        if entry.path().is_dir() {
            let platform : Platform = Platform::new(entry.path().file_name().unwrap().to_str().unwrap());
            if platform != Platform::None {
                for version_entry in read_dir(entry.path())? {
                    let version_entry = version_entry?;
                    let version = Version::from_str(version_entry.path().file_name().unwrap().to_str().unwrap());
                    if let Some(version)  = version {
                        releases.insert(Release{
                            platform : platform.clone(),
                            version : version,
                            link : "".to_string()
                        });
                    }
                }
            }
        }
    }

    Ok(releases)
}
