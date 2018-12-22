use failure::Error;

use platform_lp::{PartialPlatform, Platform};
use version_lp::Version;
use lpsettings;

use std::path::PathBuf;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;

use reqwest;
use serde_json;
use regex::Regex;
use toml;

use structs::release::Release;

// linux is the only one that can resolve without getting a full match
// on platform, these should only be lowercase!
static VALID_EXT_LINUX : [&str;3] = [ "appimage","tar.gz","tar.xz" ];
static VALID_EXT_WINDOWS : [&str;1] = [ "zip" ];
static VALID_EXT_MAC : [&str;2] = [ "zip","dmg" ];

static REGEX_VERSION_MATCH : &str = r"(\d+[-|.|_]\d+[-|.|_]\d+)";
static REPO_FILE : &str = "love_repo.toml";
static DEFAULT_LINKS : [&str;2] = [
  "https://api.bitbucket.org/2.0/repositories/rude/love/downloads",
  "https://api.bitbucket.org/2.0/repositories/snsvrno/love-linux-portable-binaries/downloads"
];

pub fn get_version_link(platform : &Platform, version : &Version) -> Result<String,Error> {
    Err(format_err!("Not Implemented"))    
}

pub fn update_local_repo(forced : bool) -> Result<(),Error> {
    if lpsettings::update::check_if_should_update("lprun.repo") || forced {
        let repo_path = get_repo_path();
        let mut links = get_repo_links();

        let mut releases : HashSet<Release> = HashSet::new();
        
        loop {
            match links.pop() {
                None => break,
                Some(link) => {
                    // do something here, to find the link
                    if let Some(additional_link) = process_bitbucket(&mut releases, &link)? { 
                        links.push(additional_link); 
                    }
                }
            }
        }

        // saves the file.

        let mut file = File::create(&repo_path)?;
        let toml_string = toml::to_string(&releases)?;

        file.write(toml_string.as_bytes())?;
    }

    lpsettings::update::set_last_update_as_now("lprun.repo")?;
    
    Ok(())
}

fn get_repo_path() -> PathBuf {
    //! gets the path of the repo local file, defaults to ~/.lovepack/repo.toml
    //! 
    //! this value can be changed by setting the correct variables use lpsettings

    let mut path = lpsettings::get_folder();
    let repo_file = lpsettings::get_value_or("lprun.repo.file", &String::from(REPO_FILE));
    path.push(format!("{}",repo_file));
    path
}

fn get_repo_links() -> Vec<String> {
    //! gets the list of links to check, (1) will use the default ones unless told not to
    //! and (2) will load additional ones if they are available.
    
    let mut links : Vec<String> = Vec::new();

    // loads the default links. can be disabled by setting the option *install.use_default_repos* to "false"
    if let lpsettings::Type::Switch(true) = lpsettings::get_value_or("lprun.repo.use_defaults",&true) {
        for default_repo in DEFAULT_LINKS.iter() { links.push(default_repo.to_string()); }
    }

    if let Ok(Some(custom_link)) = lpsettings::get_value("lprun.repo.links") {
        match custom_link {
            lpsettings::Type::Text(a_link) => links.push(a_link),
            lpsettings::Type::Array(array_link) => { 
                for member in array_link {
                    if let lpsettings::Type::Text(a_link) = member {
                        links.push(a_link);
                    }
                }
            },
            _ => ()
        }
    }

    links
}

fn process_bitbucket(mut repo_obj : &mut HashSet<Release>, url : &str) -> Result<Option<String>,Error> {
    if !url.contains("bitbucket") { return Ok(None); }

    let mut resp = reqwest::get(url)?;
    let raw_json = resp.text()?;
    let json : serde_json::Value = serde_json::from_str(&raw_json)?;

    if let Some(json_releases) = json["values"].as_array() {
        let re_version = Regex::new(REGEX_VERSION_MATCH).unwrap();
        for download in json_releases {
            if let Some(version_cap) = re_version.captures(download["name"].as_str().unwrap()) {
                match Version::from_str(version_cap.get(1).unwrap().as_str()) {
                    None => { error!("Error parsing version {:?}",version_cap.get(1).unwrap()); },
                    Some(version) => {
                        let link = download["links"]["self"]["href"].as_str().unwrap();

                        // resolves the platform, does it this way because there is some nuance it it,
                        // because some of the files don't have platforms, but the extension (like AppImage)
                        // gives it away.
                        // also some releases are installable, and we don't want those, we want the 
                        // 'portable' zipped archive release instead.
                        let platform = {

                            let mut platform_guess = Platform::new(download["name"].as_str().unwrap());
                            let mut valid = false;

                            // checks if valid linux platform
                            if platform_guess == PartialPlatform::Linux {
                                for part in &VALID_EXT_LINUX {
                                    if link.to_lowercase().contains(part) {
                                        valid = true; break;
                                    }
                                }
                            } else if platform_guess == PartialPlatform::Windows {
                                for part in &VALID_EXT_WINDOWS {
                                    if link.to_lowercase().contains(part) {
                                        valid = true; break;
                                    }
                                }
                            } else if platform_guess == PartialPlatform::Mac {
                                for part in &VALID_EXT_MAC {
                                    if link.to_lowercase().contains(part) {
                                        valid = true; break;
                                    }
                                }
                            } else if platform_guess == Platform::None {
                                for part in &VALID_EXT_LINUX {
                                    if link.to_lowercase().contains(part) {
                                        if link.to_lowercase().contains("686") || link.to_lowercase().contains("32") {
                                            platform_guess = Platform::Nix32;
                                            valid = true;
                                        } else if link.to_lowercase().contains("64") {
                                            platform_guess = Platform::Nix64;
                                            valid = true;
                                        }
                                        break;
                                    }
                                } 
                            }

                            if valid {
                                platform_guess
                            } else {
                                Platform::None
                            }
                        };

                        if platform == Platform::None {
                            error!("Error parsing platform {}",download["name"].as_str().unwrap());
                        } else {
                            let link = link.to_string();
                            let release = Release { version, platform, link };
                            info!("Found release {}",release);
                            repo_obj.insert(release);
                        }
                    }
                }
                
            }
        }
    }

    // checks if there is another page to look at.
    match json["next"].as_str() {
        Some(next) => Ok(Some(next.to_string())),
        None => Ok(None),
    }
}