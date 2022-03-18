use std::fmt;
use std::fs;
use std::path::{Path};
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
}
