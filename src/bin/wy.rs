//use clap::{App, AppSettings};
use std::path::PathBuf;
use std::env;
use std::fs;
use log::LevelFilter;
use whiley::config::Config;
use whiley::jvm::Jvm;
use whiley::{init_logging,init_whileyhome,init_classpath};

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

fn main() {
    // Initialise logging
    init_logging(LevelFilter::Info);
    // Initialise Whiley home directory
    let whileyhome = init_whileyhome();
    // Read build configuration
    let config_file = fs::read_to_string("wy.toml").expect("Error reading build configuration!");
    // Parse build configuration
    let config = Config::from_str(config_file.as_str());
    println!("PACKAGE {}",config.package.name);
    println!("VERSION {}",config.package.version);
    println!("AUTHORS {:?}",config.package.authors);    
    println!("PLATFORMS {:?}",config.build.platforms.len());
    // Initialise classpath as necessary.  This will download Jar
    // files from Maven central (if not already cached).    
    let cp = init_classpath(&whileyhome,MAVEN_DEPS);
    // Construct JVM runner
    let jvm = Jvm::new(cp,vec![("WHILEYHOME",&whileyhome)]);
    // Extract command-line arguments
    let mut args : Vec<String> = env::args().collect();
    // Replace first element (which is this program)
    args[0] = "wycli.Main".to_string();
    // Convert into Vec<&str> for exec
    let str_args : Vec<&str> = args.iter().map(String::as_str).collect();
    // Go!
    jvm.exec(&str_args);
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
