use std::error::Error;
use std::fs;
use std::path::Path;
use log::info;
use crate::config::Config;
use crate::build::{Artifact,Build};
use crate::{init_registry};

// Clean command
pub fn clean(_whileyhome: &Path) -> Result<(),Box<dyn Error>> {
    // Read build configuration
    let config_file = fs::read_to_string("wy.toml").expect("Error reading build configuration!");
    // Parse configuration
    let config = Config::from_str(config_file.as_str())?;    
   // Initialise platform registry
    let registry = init_registry();    
    // Construct build plan
    let build = Build::from_str(&config,&registry)?;
    //
    for ba in build.manifest() {
	match ba {
	    Artifact::Binary(p) => {
		info!("Deleting file {}",p.display());
		fs::remove_file(p)?;
	    }
	    _ => {
	    }
	}
    }
    //
    Ok(())
}
