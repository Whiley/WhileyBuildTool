pub mod build;
pub mod command;
pub mod config;
pub mod jvm;
pub mod maven;
pub mod platform;

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
use crate::platform::whiley;

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
	fs::create_dir(whileyhome.as_path());
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

pub fn init_classpath(whileyhome: &Path, deps : &[&str]) -> Vec<PathBuf> {
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
	classpath.push(resolver.get(mdep).unwrap());
    }
    // Done
    classpath
}

/// Initialise the default platform registry.  This basically provides
/// a mechanism for creating platform instances and running them.
pub fn init_registry<'a>() -> platform::Registry<'a> {
    let mut r = platform::Registry::new();
    // Register the Whiley platform.  This takes care of compiling
    // Whiley files into WyIL file.
    r.register("whiley",&whiley::DESCRIPTOR);
    // Done
    r
}
