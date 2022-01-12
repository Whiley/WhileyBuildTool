use std::fmt;
use std::fs;
use std::path::{Path,PathBuf};

#[derive(Clone,Debug,PartialEq)]
pub struct MavenArtifact<'a> {
    group_id : &'a str,
    artifact_id : &'a str,
    version: &'a str    
}

impl<'a> MavenArtifact<'a> {
    pub fn new(desc : &str) -> Result<MavenArtifact,()> {
	let parts = desc.split(":").collect::<Vec<&str>>();
	//
	if parts.len() != 3 {
	    Err(())
	} else {
	    Ok(MavenArtifact{group_id:parts[0],artifact_id:parts[1],version:parts[2]})
	}
    }

    pub fn to_jarname(&self) -> String {
	let mut n = String::new();
	n.push_str(self.artifact_id);
	n.push_str("-");
	n.push_str(self.version);
	n.push_str(".jar");
	n
    }
}

impl<'a> fmt::Display for MavenArtifact<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}:{}:{}",self.group_id,self.artifact_id,self.version)	
    }
}

// ==========================================================
// Resolver
// ==========================================================

#[derive(Clone,Debug,PartialEq)]
pub struct MavenResolver<'a,T: AsRef<Path>> {
    /// Path to cache root on local filesyste
    dir: T,
    /// Base URL for downloading Maven Jars (e.g. Maven Central)
    url: &'a str
}

impl<'a, T: AsRef<Path>> MavenResolver<'a,T> {
    pub fn new(dir: T, url: &'a str) -> MavenResolver<'a,T> {
	// Ensure cache directory exists
	fs::create_dir_all(dir.as_ref()).unwrap();
	MavenResolver{dir,url}
    }

    pub fn get<'b>(&self, artifact: MavenArtifact<'b>) -> Result<PathBuf,()> {
	// Determine jar name
	let mut jar = PathBuf::new();
	jar.push(self.dir.as_ref());	
	jar.push(artifact.to_jarname());
	//
	if jar.as_path().exists() {
	    // Cache hit
	    Ok(jar)
	} else {
	    println!("Cannot find {}",jar.into_os_string().into_string().unwrap());	    
	    Err(())
	}
    }
}
