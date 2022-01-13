//use clap::{App, AppSettings};
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
use whiley::jvm::Jvm;
use whiley::maven::{MavenArtifact,MavenResolver};

/// Identify the necessary dependencies (from Maven central) necessary
/// to run Whiley.  Eventually, the intention is to reduce these
/// dependencies eventually to nothing.
static MAVEN_DEPS : &'static [&str] = &[
    "commons-logging:commons-logging:1.2",
    "commons-codec:commons-codec:1.11",
    "org.apache.httpcomponents:httpcore:4.4.12",
    "org.apache.httpcomponents:httpclient:4.5.10",            
    "org.whiley:jbuildfs:1.0.1",
    "org.whiley:jmodelgen:0.4.3",
    "org.whiley:wycc:0.9.9",
    "org.whiley:wycli:0.9.9",
    "org.whiley:wyc:0.9.9",
    "org.whiley:wyjs:0.9.6",
    "org.whiley:wyboogie:0.3.4"
];

const DEFAULT_CONFIG : &str = r###"
[plugins]
wyc = "wyc.Activator"
wyjs = "wyjs.Activator"
wyboogie = "wyboogie.Activator"
"###;

/// Default URL from which to locate Maven dependencies.
const MAVEN_CENTRAL : &str = "https://repo1.maven.org/maven2/";

fn main() {
    // Initialise logging
    init_logging(LevelFilter::Info);
    // Initialise Whiley home directory
    let whileyhome = init_whileyhome();
    // Initialise classpath as necessary.  This will download Jar
    // files from Maven central (if not already cached).    
    let cp = init_classpath(&whileyhome,MAVEN_DEPS);
    // Construct JVM runner
    let jvm = Jvm::new(cp,vec![("WHILEYHOME",&whileyhome)]);
    // Extract command-line arguments
    let mut args : Vec<String> = env::args().collect();
    // Strip first element (is this program)
    args.remove(0);
    // Convert into Vec<&str> for exec
    let str_args : Vec<&str> = args.iter().map(String::as_str).collect();
    // Go!
    jvm.exec(&str_args);
}

fn init_logging(level: LevelFilter) {
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

fn init_whileyhome() -> PathBuf {
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
    let mut config = whileyhome.join("wy.toml");    
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

fn init_classpath(whileyhome: &PathBuf, deps : &[&str]) -> Vec<PathBuf> {
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
