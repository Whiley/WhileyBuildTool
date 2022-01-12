use std::error::Error;
use std::fmt;
use std::fs;
use std::fs::File;
use std::path::{Path,PathBuf};
use log::{info};
use reqwest;
use reqwest::Url;

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

    pub fn to_url(&self, base: &Url) -> Url {
	let mut s = String::new();	
	s.push_str(self.group_id.replace(".","/").as_str());
	s.push_str("/");
	s.push_str(self.artifact_id);
	s.push_str("/");
	s.push_str(self.version);
	s.push_str("/");
	s.push_str(self.to_jarname().as_str());
	// ERROR HANDLING
	base.join(&s).unwrap()
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
pub struct MavenResolver<T: AsRef<Path>> {
    /// Path to cache root on local filesyste
    dir: T,
    /// Base URL for downloading Maven Jars (e.g. Maven Central)
    url: Url
}

impl<T: AsRef<Path>> MavenResolver<T> {
    pub fn new(dir: T, url: Url) -> MavenResolver<T> {
	// Ensure cache directory exists
	fs::create_dir_all(dir.as_ref()).unwrap();
	// Done
	MavenResolver{dir,url}
    }

    pub fn get<'b>(&self, artifact: MavenArtifact<'b>) -> Result<PathBuf,Box<dyn Error>> {
	// Determine jar name
	let mut jar = PathBuf::new();
	jar.push(self.dir.as_ref());	
	jar.push(artifact.to_jarname());
	//
	if !jar.as_path().exists() {
	    // Cache miss, try to download
	    let url = artifact.to_url(&self.url);
	    info!("Downloading {}",url.as_str());
	    let response = reqwest::blocking::get(url)?.bytes()?;
	    fs::write(jar.as_path(),response)?;
	}
	//
	Ok(jar)
    }
}
