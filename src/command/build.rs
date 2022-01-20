use std::path::Path;
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
	    // TODO: Determine platform state
            ps.push(init.apply(config)?);
        }
	// Done
	return Ok(Build{name,authors,version,platforms:ps});
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
