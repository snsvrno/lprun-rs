use platform_lp::Platform;
use version_lp::Version;

use std::fmt;
use std::collections::HashSet;

/// Internal struct to describe a LOVE release.
#[cfg(feature = "cli")]
#[derive(Hash,Eq,PartialEq,Serialize,Deserialize,SmartHash)]
pub struct Release {
    pub version : Version,
    pub platform: Platform,
    pub link : String,
}

#[cfg(not(feature = "cli"))]
#[derive(Hash,Eq,PartialEq,Serialize,Deserialize)]
pub struct Release {
    pub version : Version,
    pub platform: Platform,
    pub link : String,
}

impl fmt::Display for Release {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}-{}",self.platform,self.version)
    }
}

impl std::cmp::Ord for Release {
    fn cmp(&self, other: &Release) -> std::cmp::Ordering {
        self.version.cmp(&other.version)
    }
}

impl std::cmp::PartialOrd for Release {
    fn partial_cmp(&self, other: &Release) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// created this struct because serializing and deserializing 
/// a HashSet doesn't work.
#[derive(Hash,Eq,PartialEq,Serialize,Deserialize)]
pub struct ReleaseExporter {
    pub releases : Vec<Release>,
}

impl ReleaseExporter {
    pub fn from_release(mut releases : HashSet<Release>) -> ReleaseExporter {
        let mut vec : Vec<Release>= Vec::new();

        for release in releases.drain() {
            vec.push(release);
        }

        ReleaseExporter {
            releases : vec,
        }
    }

    pub fn to_release(mut self) -> HashSet<Release> {
        let mut hash : HashSet<Release> = HashSet::new();

        loop {
            match self.releases.pop() {
                Some(release) => { hash.insert(release); },
                None => { break; },
            }
        }

        hash
    }
}