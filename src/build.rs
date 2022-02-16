use std::error;
use std::fs::{File,read_to_string};
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
// Result
// ===================================================================

#[derive(Debug)]
pub enum Kind {
    /// Indicates a warning of some kind
    Warning,
    /// Indicates a syntax error of some kind
    SyntaxError,
    /// Indicates an internal failure of some kind
    InternalFailure
}

#[derive(Debug)]
pub struct Marker {
    kind: Kind,
    path: PathBuf,
    start: usize,
    end: usize,
    message: String
}

impl Marker {
    pub fn new(kind: Kind, path: PathBuf, start: usize, end: usize, message: String) -> Self {
	Marker{kind,path,start,end,message}
    }
    /// Determine enclosing line information for the given marker
    pub fn enclosing_line(&self) -> Result<Line,Box<dyn error::Error>> {
	// Read marked file
	let contents = read_to_string(self.path.as_path())?;
	// Split into lines
	let mut offset = 0;
	let mut line = 1;
	//
	for l in contents.lines() {
	    if self.start < offset + l.len() {
                // Determine column difference
                offset = self.start - offset;
		// Return line
		return Ok(Line{offset,line,contents:l.to_string()});
	    }
	    line = line + 1;
	    offset = offset + l.len()
	}
	// Temporary hack
	panic!("No enclosing line!");
    }
}

pub struct Line {
    pub offset: usize,
    pub line: usize,
    pub contents: String
}

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
    pub fn from_str<'a>(config: &Config, whileyhome: &Path, registry: &'a platform::Registry<'a>) -> Result<Build,Error> {
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
            ps.push(init.apply(config,whileyhome)?);
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
    pub fn run(&self, whileyhome: &Path) -> Result<(),Box<dyn error::Error>> {
	// Execute each platform in sequence.
	for p in &self.platforms {
	    let markers = match p {
		Instance::Java(i) => {
		    self.run_java(i.as_ref(),whileyhome)
		},
		Instance::Rust(_) => {
		    todo!("Rust platforms not currently supported")
		}
	    };
            if markers.len() > 0 {
	        for m in markers {
		    // Determine enclosing line!
		    let l = m.enclosing_line()?;
		    let f = m.path.into_os_string().into_string().unwrap();
		    // Print out the error message
		    println!("{}:{}:{}",f,l.line,m.message);
		    // Print out the line highlight
		    println!("{}",l.contents);
		    let padding = " ".repeat(m.start);
		    let highlight = "^".repeat(m.end - m.start + 1);
		    println!("{}{}",padding,highlight);
	        }
                break;
            }
	}
	//
	Ok(())
    }

    /// Run a Java platform
    fn run_java(&self, i: &dyn JavaInstance, whileyhome: &Path) -> Vec<Marker> {
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
        let output = jvm.exec(&str_args);
	// Post process the response
	i.process(output.as_str())
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
	    // Push items from instance manifest
	    artifacts.extend(i.manifest());
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
