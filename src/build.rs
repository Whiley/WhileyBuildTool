use crate::config::{Config,Key,Error};
use crate::platform;

// ===================================================================
// Build
// ===================================================================

static PACKAGE_NAME : Key = Key::new(&["package","name"]);
static PACKAGE_AUTHORS : Key = Key::new(&["package","authors"]);
static PACKAGE_VERSION : Key = Key::new(&["package","version"]);
static BUILD_PLATFORMS : Key = Key::new(&["build","platforms"]);

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
    pub fn run(&self) {
	todo!("IMPLEMENT ME!");
    }
}
