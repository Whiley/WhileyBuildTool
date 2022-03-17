use std::error::Error;
use std::fs::{File,read_to_string};
use std::path::{Path,PathBuf};
use std::io::{Read,Write,Seek,copy};
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
    let build = Build::from_str(&config,whileyhome,&registry)?;
    // Construct zip file
    let pkg = format!("{}-v{}.zip",build.name,build.version);
    let path = get_pkg_path(whileyhome,&pkg);
    let zipfile = File::create(path).unwrap();
    let mut zip = zip::ZipWriter::new(zipfile);
    //
    for ba in build.manifest() {
	// Extract path from artifact
	match ba {
	    Artifact::SourceFolder(p) => {
		info!("Packaging source folder {}",p.display());
		zip.add_directory(p.to_str().unwrap(), Default::default())?;
	    }	    
	    Artifact::SourceFile(p) => {
		info!("Packaging source file {}",p.display());
		add_file(&p,&mut zip)?;
	    }	    
	    Artifact::BinaryFolder(p) => {
		info!("Packaging binary folder {}",p.display());
		zip.add_directory(p.to_str().unwrap(), Default::default())?;
	    }
	    Artifact::BinaryFile(p) => {
		info!("Packaging binary file {}",p.display());
		add_file(&p,&mut zip)?;		
	    }
	};
    }
    //
    zip.finish()?;
    info!("Installed {} ...",pkg);    
    Ok(())
}

fn add_file<T>(buf: &PathBuf, zip: &mut zip::ZipWriter<T>)  -> Result<(),Box<dyn Error>>
where T: Read + Write + Seek
{
    let path = buf.as_path();
    // Create zip entry
    let mut file = File::open(path)?;
    // Start Zip entry
    zip.start_file(path.to_str().unwrap(), Default::default())?;
    // Copy all data over
    copy(&mut file, zip)?;
    // Done
    Ok(())
}

fn get_pkg_path(whileyhome: &Path, name: &String) -> PathBuf {
    let mut buf = PathBuf::new();
    buf.push(whileyhome);
    buf.push(REPOSITORY_NAME);
    buf.push(name);
    return buf;
}
