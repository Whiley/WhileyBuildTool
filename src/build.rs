use std::path::Path;
use std::path::PathBuf;
use log::{info};
use crate::{init_classpath};
use crate::config::{Config,Key,Error};
use crate::jvm::{Jvm};
use crate::platform;
use crate::platform::{Instance,JavaInstance};

// ===================================================================
// Keys
// ===================================================================

pub static PACKAGE_NAME : Key = Key::new(&["package","name"]);
pub static PACKAGE_AUTHORS : Key = Key::new(&["package","authors"]);
pub static PACKAGE_VERSION : Key = Key::new(&["package","version"]);
pub static BUILD_PLATFORMS : Key = Key::new(&["build","platforms"]);

// ===================================================================
// Build
// ===================================================================

/// Identifies meta-data about the package in question, such its name,
/// version, etc.
pub struct Build {    
    pub name: String,
    pub authors: Vec<String>,
    pub version: String,
    /// Identifies what build platforms should be used to build the
    /// package.    
    pub platforms: Vec<platform::Instance>    
}

impl Build {
    /// Parse a give string into a build configuration.
    pub fn from_str<'a>(config: &Config, registry: &'a platform::Registry<'a>) -> Result<Build,Error> {
        // Extract all required keys
        let name = config.get_string(&PACKAGE_NAME)?;
        let authors = config.get_string_array(&PACKAGE_AUTHORS)?;
        let version = config.get_string(&PACKAGE_VERSION)?;
        let platforms = config.get_string_array(&BUILD_PLATFORMS)?;        
        // Construct build information
        let mut ps = Vec::new();        
        for p in &platforms {
            let init = match registry.get(p) {
                None => {
                    return Err(Error::UnknownPlatform(p.to_string()));
                }
                Some(v) => v
            };
            ps.push(init.apply(config)?);
        }
	// Done
	return Ok(Build{name,authors,version,platforms:ps});
    }

    /// Determine the list of know build artifacts.  This includes
    /// source files, binary files and more.    
    pub fn manifest(&self) -> Manifest {
	Manifest::new(self)
    }
    
    /// Run the given build.
    pub fn run(&self, whileyhome: &Path) {
	// Execute each platform in sequence.
	for p in &self.platforms {
	    match p {
		Instance::Java(i) => {
		    self.run_java(i.as_ref(),whileyhome)
		},
		Instance::Rust(_) => {
		    todo!("Rust platforms not currently supported")
		}
	    }
	}
    }

    /// Run a Java platform
    fn run_java(&self, i: &dyn JavaInstance, whileyhome: &Path) {
	// Initialise classpath as necessary.  This will download Jar
	// files from Maven central (if not already cached).    
	let cp = init_classpath(&whileyhome,i.dependencies());
        // Construct JVM runner
        let jvm = Jvm::new(cp,vec![("WHILEYHOME",&whileyhome)]);
        // Construct command-line arguments
        let args : Vec<String> = i.arguments();
        // Convert into Vec<&str> for exec
        let str_args : Vec<&str> = args.iter().map(String::as_str).collect();
        //
        info!("Executing java {:?}",str_args);
        // Go!
        jvm.exec(&str_args);
    }    
}

// ===================================================================
// Manifest
// ===================================================================

#[derive(Debug)]
pub enum Artifact {
    Source(PathBuf),
    Binary(PathBuf)    
}

pub struct Manifest {
    artifacts: Vec<Artifact>
}

impl Manifest {
    pub fn new(b: &Build) -> Manifest {
	let mut artifacts = Vec::new();
	artifacts.push(Artifact::Source(PathBuf::from("wy.toml")));
	// Iterator platforms looking for artifacts
	for i in &b.platforms {
	    for j in i.manifest() {
		artifacts.push(j);
	    }
	}
	//
	Manifest{artifacts}
    }
}

impl IntoIterator for Manifest {
    type Item = Artifact;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    
    fn into_iter(self) -> Self::IntoIter {
	self.artifacts.into_iter()
    }
}
