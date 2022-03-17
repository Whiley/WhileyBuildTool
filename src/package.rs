use std::error::Error;
use std::fmt;
use std::fs;
use std::path::{Path,PathBuf};
use log::{info};
use reqwest;
use reqwest::Url;

// ================================================================
// Dependency
// ================================================================

/// Identifies
pub struct Dependency {
    name: String,
    version: String
}

impl Dependency {
    pub fn new(name: String, version: String) -> Self {
        Dependency{name,version}
    }
    pub fn to_zipname(&self) -> String {
        format!("{}-v{}.zip",self.name,self.version)
    }

    pub fn to_url(&self, base: &Url) -> Url {
        let n = format!("{}/{}/{}",self.name,self.version,self.to_zipname());
	base.join(&n).unwrap()
    }
}

impl fmt::Display for Dependency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}-{}",self.name,self.version)
    }
}

// ================================================================
// Package Resolver
// ================================================================

#[derive(Clone,Debug,PartialEq)]
pub struct PackageResolver<T: AsRef<Path>> {
    /// Path to cache root on local filesyste
    dir: T,
    /// Base URL for downloading packages
    url: Url
}

impl<T: AsRef<Path>> PackageResolver<T> {
    /// Construct a package resolver which stores cached files in a
    /// given filesystem directory, and downloads them from a given
    /// base URL.
    pub fn new(dir: T, url: Url) -> Self {
	// Ensure cache directory exists
	fs::create_dir_all(dir.as_ref()).unwrap();
	// Done
	PackageResolver{dir,url}
    }

    /// Resolve a given set of dependencies.  This is a non-trivial
    /// process as we identify all transitive dependencies, and
    /// determine a coherent set which matches all versioning
    /// constraints (if one exists).
    pub fn resolve(&self, deps : &[Dependency]) -> Result<(),Box<dyn Error>> {
        for dep in deps {
            self.get(&dep)?;
        }
        // Done
        Ok(())
    }

    pub fn get<'b>(&self, dep: &Dependency) -> Result<PathBuf,Box<dyn Error>> {
	// Determine dependency location
	let mut zip = PathBuf::new();
	zip.push(self.dir.as_ref());
	zip.push(dep.to_zipname());
	//
	if !zip.as_path().exists() {
	    // Cache miss, try to download
	    let url = dep.to_url(&self.url);
	    info!("Downloading {}",url.as_str());
	    let response = reqwest::blocking::get(url)?.bytes()?;
	    fs::write(zip.as_path(),response)?;
	}
	//
	Ok(zip)
    }
}
