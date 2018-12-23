use platform_lp::Platform;
use version_lp::Version;
use std::fmt;

use std::collections::HashSet;

#[derive(Hash,Eq,PartialEq,Serialize,Deserialize,SmartHash)]
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