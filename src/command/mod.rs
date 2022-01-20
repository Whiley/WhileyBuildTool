mod build;

use std::error::Error;
use std::fs;
use std::path::Path;
use log::info;
use crate::config::Config;
use crate::{init_registry};

// Build command
use build::Build;
pub use build::{PACKAGE_NAME,PACKAGE_AUTHORS,PACKAGE_VERSION};
pub use build::{BUILD_PLATFORMS};

pub fn build(whileyhome: &Path) -> Result<(),Box<dyn Error>> {
    // Read build configuration
    let config_file = fs::read_to_string("wy.toml").expect("Error reading build configuration!");
    // Parse configuration
    let config = Config::from_str(config_file.as_str())?;    
   // Initialise platform registry
    let registry = init_registry();    
    // Construct build plan
    let build = Build::from_str(&config,&registry)?;
    // Go!
    build.run(whileyhome);
    //
    Ok(())
}

// Clean command

// Help command

// Init command

const DEFAULT_CONFIG : &str = r###"[package]
name="main"
authors=["Joe Bloggs"]
version="0.1.0"

[build]
platforms=["whiley"]

[dependencies]
std="0.3.2"
"###;

const DEFAULT_MAIN : &str = r###"
import std::io

public export method main():
    io::println("Hello World")
"###;

pub fn init(_whileyhome: &Path) -> Result<(),Box<dyn Error>> {
    let config = Path::new("wy.toml");    
    let src = Path::new("src");
    let main = Path::new("src/main.whiley");
    // Write default configuration
    if !main.exists() {
	info!("Creating file {} ...",config.display());	
	fs::write(config,DEFAULT_CONFIG)?;
    }       
    // Create src directory
    if !src.exists() {
	info!("Creating directory {} ...",src.display());
	fs::create_dir(src)?;
    }
    // Write initial source file
    if !main.exists() {
	info!("Creating file {} ...",main.display());	
	fs::write(main,DEFAULT_MAIN)?;
    }   
    Ok(())
}

// Install command


