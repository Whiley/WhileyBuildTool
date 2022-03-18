use std::error;
use std::fs::{read_to_string,create_dir_all};
use std::path::Path;
use std::path::PathBuf;
use log::{info};
use reqwest::Url;
use crate::{init_classpath};
use crate::util;
use crate::config::{Config,Key,Error};
use crate::jvm::{Jvm};
use crate::package::{Dependency, PackageResolver};
use crate::platform;
use crate::platform::{Instance,JavaInstance};

// ===================================================================
// Keys
// ===================================================================

pub static PACKAGE_NAME : Key = Key::new(&["package","name"]);
pub static PACKAGE_AUTHORS : Key = Key::new(&["package","authors"]);
pub static PACKAGE_VERSION : Key = Key::new(&["package","version"]);
pub static BUILD_PLATFORMS : Key = Key::new(&["build","platforms"]);
pub static DEPENDENCIES : Key = Key::new(&["dependencies"]);

/// Default URL from which to resolve package dependencies.
const PACKAGE_CENTRAL : &str = "https://github.com/Whiley/Repository/raw/master/";

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
	let mut line = 1;
	//
	for l in util::line_offsets(contents.as_str()) {
	    if l.contains(self.start) {
		return Ok(Line{offset:l.start,line,contents:l.as_str().to_string()});
	    }
	    line = line + 1;
	}
	// Temporary hack
	panic!("No enclosing line!");
    }
}

pub struct Line {
    /// Offset of this line in the original file
    pub offset: usize,
    /// Line number for this line
    pub line: usize,
    /// Contents of this line
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
    pub platforms: Vec<platform::Instance>,
    /// Identify dependencies for this build
    pub dependencies: Vec<Dependency>
}

impl Build {
    /// Parse a give string into a build configuration.
    pub fn from_str<'a>(config: &Config, whileyhome: &Path, registry: &'a platform::Registry<'a>) -> Result<Build,Error> {
        // Extract all required keys
        let name = config.get_string(&PACKAGE_NAME)?;
        let authors = config.get_string_array(&PACKAGE_AUTHORS)?;
        let version = config.get_string(&PACKAGE_VERSION)?;
        let platforms = config.get_string_array(&BUILD_PLATFORMS)?;
	let deps = config.get_strings(&DEPENDENCIES).unwrap_or(Vec::new());
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
        // Map deps
        let dependencies = deps.into_iter().map(|(k,v)| Dependency::new(k,v)).collect();
	// Done
	return Ok(Build{name,authors,version,platforms:ps,dependencies});
    }

    /// Determine the list of know build artifacts.  This includes
    /// source files, binary files and more.
    pub fn manifest(&self) -> Manifest {
	Manifest::new(self)
    }

    /// Run the given build.
    pub fn run(&self, whileyhome: &Path) -> Result<bool,Box<dyn error::Error>> {
	// Perform startup initialisation(s)
	self.initialise(whileyhome)?;
	// Execute each platform in sequence.
	for p in &self.platforms {
	    // Execute plugin
	    let result = match p {
		Instance::Java(i) => {
		    self.run_java(i.as_ref(),whileyhome)
		},
		Instance::Rust(_) => {
		    todo!("Rust platforms not currently supported")
		}
	    };
	    // Decode output
	    match result {
		Ok(markers) => {
		    if markers.len() > 0 {
			for m in markers {
			    // Determine enclosing line!
			    let l = m.enclosing_line()?;
			    let f = m.path.into_os_string().into_string().unwrap();
			    // Print out the error message
			    println!("{}:{}:{}",f,l.line,m.message);
			    // Print out the line highlight
			    println!("{}",l.contents);
			    let padding = " ".repeat(m.start - l.offset);
			    let highlight = "^".repeat(m.end - m.start + 1);
			    println!("{}{}",padding,highlight);
			}
			// Fail
			return Ok(false);
		    }
		}
		Err(out) => {
		    println!("{}",out);
		    // Failure
		    return Ok(false);
		}
            }
	}
	// Success
	Ok(true)
    }

    /// Run a Java platform
    fn run_java(&self, i: &dyn JavaInstance, whileyhome: &Path) -> Result<Vec<Marker>,platform::Error> {
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

    /// Perform necessary initialisation for this build
    /// (e.g. downloading dependencies, etc).
    fn initialise(&self, whileyhome: &Path) -> Result<(),Box<dyn error::Error>> {
        self.create_binary_folders()?;
        //
        self.resolve_packages(whileyhome);
        // Done
        Ok(())
    }

    /// Create binary folder(s) as necessary to store generated files.
    fn create_binary_folders(&self) -> Result<(),Box<dyn error::Error>> {
	// Construct local folders as necessary.
	for ba in self.manifest() {
	    // Construct any missing binary folders.
	    match ba {
		Artifact::BinaryFolder(p) => {
		    if !p.as_path().exists() {
			info!("Making binary folder {}",p.display());
			create_dir_all(p)?;
		    }
		}
		_ => {
		}
	    };
	}
        // Done
        Ok(())
    }

    /// Resolve all packages specified as dependencies.  This means
    /// determining appropriate versions and, potentially, downloading
    /// them.
    fn resolve_packages(&self, whileyhome: &Path) {
        // Append repository into Whiley home
        let mut repo = PathBuf::from(whileyhome);
        repo.push("repository");
        // Parse the base URL
        let base_url = Url::parse(PACKAGE_CENTRAL).unwrap();
        // Construct Package resolver
        let resolver = PackageResolver::new(repo, base_url);
	// Resolve package dependencies
        resolver.resolve(&self.dependencies);
    }
}

// ===================================================================
// Manifest
// ===================================================================

#[derive(Debug)]
pub enum Artifact {
    SourceFile(PathBuf),
    SourceFolder(PathBuf),
    BinaryFile(PathBuf),
    BinaryFolder(PathBuf)
}

pub struct Manifest {
    artifacts: Vec<Artifact>
}

impl Manifest {
    pub fn new(b: &Build) -> Manifest {
	let mut artifacts = Vec::new();
	artifacts.push(Artifact::SourceFile(PathBuf::from("wy.toml")));
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
