use platform_lp::Platform;
use version_lp::Version;
use std::fmt;

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