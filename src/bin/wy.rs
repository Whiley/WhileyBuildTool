use clap::{arg, App, AppSettings};
use std::error::Error;
use std::path::Path;
use std::fs;
use log::LevelFilter;
use whiley::config::Config;
use whiley::command::Build;
use whiley::{init_logging,init_whileyhome,init_registry};

fn main() -> Result<(),Box<dyn Error>> {
    // Parse command-line arguments
    let matches = App::new("wy")
	.about("Whiley Build Tool")
	.version("0.6.0")	
	.setting(AppSettings::SubcommandRequiredElseHelp)
	.arg(arg!(--verbose "Show verbose output"))
	.subcommand(
	    App::new("build").about("Build local package(s)"))
	.get_matches();
    // Extract top-level flags
    let verbose = matches.is_present("verbose");    
    // Initialise logging
    if verbose {
	init_logging(LevelFilter::Info);
    }
    // Initialise Whiley home directory
    let whileyhome = init_whileyhome();
    // Read build configuration
    let config_file = fs::read_to_string("wy.toml").expect("Error reading build configuration!");
    // Parse configuration
    let config = Config::from_str(config_file.as_str())?;
    // Dispatch on outcome
    match matches.subcommand() {
	Some(("build", _)) => build(&config,&whileyhome),
	_ => unreachable!()
    }
}

fn build(config: &Config, whileyhome: &Path) -> Result<(),Box<dyn Error>> {
   // Initialise platform registry
    let registry = init_registry();    
    // Construct build plan
    let build = Build::from_str(&config,&registry)?;
    // Go!
    build.run(whileyhome);
    //
    Ok(())
}
