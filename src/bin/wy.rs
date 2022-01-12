//use clap::{App, AppSettings};
use std::path::PathBuf;
use dirs;
use log::LevelFilter;
use log::{info};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Root};
use reqwest::Url;
use whiley::jvm::Jvm;
use whiley::maven::{MavenArtifact,MavenResolver};

/// Identify the necessary dependencies (from Maven central) necessary
/// to run Whiley.  Eventually, the intention is to reduce these
/// dependencies eventually to nothing.
static MAVEN_DEPS : &'static [&str] = &[
    "org.apache.httpcomponents:httpclient:4.5.13",            
    "org.whiley:jbuildfs:1.0.1",
    "org.whiley:jmodelgen:0.4.3",
    "org.whiley:wycc:0.9.9",
    "org.whiley:wycli:0.9.9",
    "org.whiley:wyc:0.9.9",
    "org.whiley:wyjs:0.9.6",
    "org.whiley:wyboogie:0.3.4"
];

/// Default URL from which to locate Maven dependencies.
const MAVEN_CENTRAL : &str = "https://repo1.maven.org/maven2/";

fn main() {
    // Initialise logging
    init_logging(LevelFilter::Info);
    // Initialise Whiley home directory
    let whileyhome = init_whileyhome();
    // Initialise classpath as necessary.  This will download Jar
    // files from Maven central (if not already cached).    
    let cp = init_classpath(whileyhome,MAVEN_DEPS);
    // Construct JVM runner
    let jvm = Jvm::new(cp);
    // Go!
    jvm.exec(&["--version"]);
}

fn init_logging(level: LevelFilter) {
    let stdout = ConsoleAppender::builder().build();
    //
    let config =
    Config::builder().appender(Appender::builder().build("stdout",
    Box::new(stdout))).build(Root::builder().appender("stdout").build(level))
	.unwrap();
    //
    let _handle = log4rs::init_config(config).unwrap();    
}

fn init_whileyhome() -> PathBuf {
    // Determine Whiley home directory ($HOME/.whiley)
    let mut home = dirs::home_dir().unwrap();
    home.push(".whiley");
    let whileyhome = home.as_path();
    // Create Whiley home directory (if doesn't exist)
    if !whileyhome.exists() {
	info!("Creating directory {} ...",whileyhome.display());
    }
    // Done
    home
}

fn init_classpath(mut whileyhome: PathBuf, deps : &[&str]) -> Vec<PathBuf> {
    // Append maven into Whiley home
    whileyhome.push("maven");
    // Parse the base URL
    let base_url = Url::parse(MAVEN_CENTRAL).unwrap();
    // Construct Maven resolver
    let resolver = MavenResolver::new(whileyhome, base_url);
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

// pub fn main() {
//     // Parse command-line arguments
//     let matches = App::new("wy")
// 	.about("Whiley's Build Tool and Package Manager")
// 	.version("0.6.0")
// 	.setting(AppSettings::SubcommandRequiredElseHelp)
// 	.subcommand(
// 	    App::new("build").about("Build local package(s)"))
// 	.subcommand(
// 	    App::new("clean").about("Remove all generated build artifact(s)"))	
// 	.get_matches();
//     // Dispatch on outcome
//     match matches.subcommand() {
// 	("build", Some(_)) => {
// 	    println!("Build not implemented yet!");
// 	}
// 	("clean", Some(_)) => {
// 	    println!("Clean not implemented yet!");
// 	}
// 	_ => unreachable!()
//     }
//     //
//}
