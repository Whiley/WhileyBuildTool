use std::fmt;

/// Identifies
pub struct Dependency {
    name: String,
    version: String
}

impl Dependency {
    pub fn new(name: String, version: String) -> Self {
        Dependency{name,version}
    }
}

impl fmt::Display for Dependency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}-{}",self.name,self.version)
    }
}
