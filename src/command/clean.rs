use std::error::Error;
use std::fs;
use std::path::Path;
use log::info;
use crate::config::Config;
use crate::build::{Artifact,Build};
use crate::{init_registry};

// Clean command
pub fn clean(whileyhome: &Path) -> Result<(),Box<dyn Error>> {
    // Read build configuration
    let config_file = fs::read_to_string("wy.toml").expect("Error reading build configuration!");
    // Parse configuration
    let config = Config::from_str(config_file.as_str())?;    
   // Initialise platform registry
    let registry = init_registry();    
    // Construct build plan
    let build = Build::from_str(&config,whileyhome,&registry)?;
    // Clean all folders
    for ba in build.manifest() {
	match ba {
	    Artifact::BinaryFolder(p) => {
		if p.as_path().exists() {
		    info!("Removing folder {}",p.display());
		    fs::remove_dir_all(p)?;
		}
	    }
	    _ => {
	    }
	}
    }    
    //
    Ok(())
}

