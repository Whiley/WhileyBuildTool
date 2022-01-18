pub mod jvm;
pub mod maven;
pub mod config;
pub mod platform;
pub mod platforms;

use std::path::PathBuf;
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
use crate::platform::PlatformRegistry;
use crate::platforms::whiley::WHILEY_PLATFORM;

/// Default URL from which to locate Maven dependencies.
const MAVEN_CENTRAL : &str = "https://repo1.maven.org/maven2/";

const DEFAULT_CONFIG : &str = r###"
[plugins]
wyc = "wyc.Activator"
wyjs = "wyjs.Activator"
wyboogie = "wyboogie.Activator"
"###;

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
    // Construct global configuration file
    let config = whileyhome.join("wy.toml");    
    // Initialise global configuration (if doesn't exist)
    if !config.as_path().exists() {
	info!("Creating global configuration {} ...",config.display());	
	fs::write(config.as_path(),DEFAULT_CONFIG).unwrap();
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

pub fn init_classpath(whileyhome: &PathBuf, deps : &[&str]) -> Vec<PathBuf> {
    // Append maven into Whiley home
    let mut mavenhome = whileyhome.clone();
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

pub fn init_registry<'a>() -> PlatformRegistry<'a> {
    let mut r = PlatformRegistry::new();
    r.register(&WHILEY_PLATFORM);
    r
}
