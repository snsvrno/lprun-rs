use failure::Error;

use platform_lp::Platform;
use version_lp::Version;
use lpsettings;

use std::path::PathBuf;

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
        
        loop {
            match links.pop() {
                None => break,
                Some(link) => {
                    // do something here, to find the link
                }
            }
        }
    }

    lpsettings::update::set_last_update_as_now("lprun.repo")?;
    
    Ok(())
}

fn get_repo_path() -> PathBuf {
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