pub mod build;
pub mod command;
pub mod config;
pub mod jvm;
pub mod maven;
pub mod package;
pub mod platform;
mod util;

use std::error::Error;
use std::path::{Path,PathBuf};
use std::env;
use std::fs;
use dirs;
use log::LevelFilter;
use log::{info};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::{PatternEncoder};
use reqwest::Url;
use crate::maven::{MavenArtifact,MavenResolver};
use crate::platform::{whiley,javascript,boogie};

/// Default URL from which to locate Maven dependencies.
const MAVEN_CENTRAL : &str = "https://repo1.maven.org/maven2/";

pub fn init_logging(level: LevelFilter) {
    let encoder = PatternEncoder::new("[{l}] {m}{n}");
    //
    let stdout = ConsoleAppender::builder()
	.encoder(Box::new(encoder))
	.build();
    //
    let config = Config::builder()
	.appender(Appender::builder().build("stdout", Box::new(stdout)))
	.build(Root::builder().appender("stdout").build(level))
	.unwrap();
    //
    let _handle = log4rs::init_config(config).unwrap();
}

/// Initialise the home directory for this tool.  This is where the
/// package repository and the maven cache live, along with other
/// configuration files as needed.  This first checks whether or not
/// the WHILEYHOME environment variable is specified, in which case it
/// uses that.
pub fn init_whileyhome() -> PathBuf {
    // Determine Whiley home directory ($HOME/.whiley)
    let whileyhome = match env::var("WHILEYHOME") {
	Ok(val) => {
	    // WHILEYHOME defined, so use it without questions.
	    PathBuf::from(val)
	}
	Err(_) => {
	    // WHILEYHOME not defined, therefore use default.
	    default_whileyhome()
	}
    };
    info!("WHILEYHOME is {}",whileyhome.as_path().to_str().unwrap());
    // Create Whiley home directory (if doesn't exist)
    if !whileyhome.as_path().exists() {
	info!("Creating directory {} ...",whileyhome.display());
	fs::create_dir(whileyhome.as_path()).unwrap();
    }
    // Done
    whileyhome
}

/// Construct a default path for WHILEYHOME which exists relative to
/// the user's home directory.
fn default_whileyhome() -> PathBuf {
    let mut p = dirs::home_dir().unwrap();
    p.push(".whiley");
    p
}

/// Initialise classpath for a given set of Maven dependencies.  This
/// means resolving those dependencies as necessary from Maven
/// central.
pub fn init_classpath(whileyhome: &Path, deps : &[&str]) -> Result<Vec<PathBuf>,Box<dyn Error>> {
    // Append maven into Whiley home
    let mut mavenhome = PathBuf::from(whileyhome);
    mavenhome.push("maven");
    // Parse the base URL
    let base_url = Url::parse(MAVEN_CENTRAL).unwrap();
    // Construct Maven resolver
    let resolver = MavenResolver::new(mavenhome, base_url);
    // Begin
    let mut classpath = Vec::new();
    //
    for dep in deps {
    	let mdep = MavenArtifact::new(dep).unwrap();
	classpath.push(resolver.get(mdep)?);
    }
    // Done
    Ok(classpath)
}

/// Initialise the default platform registry.  This basically provides
/// a mechanism for creating platform instances and running them.
pub fn init_registry<'a>() -> platform::Registry<'a> {
    let mut r = platform::Registry::new();
    // Register the Whiley platform.  This takes care of compiling
    // Whiley files into WyIL file.
    r.register("whiley",&whiley::DESCRIPTOR);
    // Register the JavaScript platform which is responsible for compiling WyIL files into JavaScript files.
    r.register("js",&javascript::DESCRIPTOR);
    // Register the Boogie platform which is responsible for compiling WyIL files into BPL files.
    r.register("boogie",&boogie::DESCRIPTOR);
    // Done
    r
}
