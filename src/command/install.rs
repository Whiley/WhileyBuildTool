use std::error::Error;
use std::fs::{File,read_to_string};
use std::path::{Path,PathBuf};
use log::info;
use crate::config::Config;
use crate::build::{Artifact,Build};
use crate::{init_registry};

const REPOSITORY_NAME : &'static str = "repository";

pub fn install(whileyhome: &Path) -> Result<(),Box<dyn Error>> {
    // Read build configuration
    let config_file = read_to_string("wy.toml")?;
    // Parse configuration
    let config = Config::from_str(config_file.as_str())?;    
   // Initialise platform registry
    let registry = init_registry();    
    // Construct build plan
    let build = Build::from_str(&config,&registry)?;
    // Construct zip file
    let pkg = format!("{}-{}.zip",build.name,build.version);
    let path = get_pkg_path(whileyhome,&pkg);
    let zipfile = File::create(path).unwrap();
    let mut zip = zip::ZipWriter::new(zipfile);
    //
    for ba in build.manifest() {
	// Extract path from artifact
	let buf = match ba {
	    Artifact::Binary(p) => {
		info!("Packaging binary file {}",p.display());
		p		
	    }
	    Artifact::Source(p) => {
		info!("Packaging source file {}",p.display());
		p
	    }
	};
	let path = buf.as_path();
	// Create zip entry
	let mut file = File::open(path)?;
	// Start Zip entry
	zip.start_file(path.to_str().unwrap(), Default::default())?;
	// Copy all data over
	std::io::copy(&mut file, &mut zip)?;	
    }
    //
    zip.finish()?;
    info!("Installed {} ...",pkg);    
    Ok(())
}

fn get_pkg_path(whileyhome: &Path, name: &String) -> PathBuf {
    let mut buf = PathBuf::new();
    buf.push(whileyhome);
    buf.push(REPOSITORY_NAME);
    buf.push(name);
    return buf;
}
