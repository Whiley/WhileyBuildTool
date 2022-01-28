use std::error::Error;
use std::fs;
use std::path::Path;
use crate::config::Config;
use crate::build::{Build};
use crate::{init_registry};

// Build command

pub fn build(whileyhome: &Path) -> Result<(),Box<dyn Error>> {
    // Read build configuration
    let config_file = fs::read_to_string("wy.toml").expect("Error reading build configuration!");
    // Parse configuration
    let config = Config::from_str(config_file.as_str())?;    
   // Initialise platform registry
    let registry = init_registry();    
    // Construct build plan
    let build = Build::from_str(&config,whileyhome,&registry)?;
    // Go!
    build.run(whileyhome)?;
    //
    Ok(())
}
