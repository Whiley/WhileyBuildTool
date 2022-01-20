use clap::{arg, App, AppSettings};
use std::error::Error;
use log::LevelFilter;
use whiley::command::{build,clean,init};
use whiley::{init_logging,init_whileyhome};

fn main() -> Result<(),Box<dyn Error>> {
    // Parse command-line arguments
    let matches = App::new("wy")
	.about("Whiley Build Tool")
	.version("0.6.0")	
	.setting(AppSettings::SubcommandRequiredElseHelp)
	.arg(arg!(--verbose "Show verbose output"))
	.subcommand(
	    App::new("build").about("Build local package(s)"))
	.subcommand(
	    App::new("init").about("Create a new Whiley package in an existing directory"))
	.subcommand(
	    App::new("clean").about("Remove all generated (binary) files"))	
	.get_matches();
    // Extract top-level flags
    let verbose = matches.is_present("verbose");    
    // Initialise logging
    if verbose {
	init_logging(LevelFilter::Info);
    }
    // Initialise Whiley home directory
    let whileyhome = init_whileyhome();
    // Dispatch on outcome
    match matches.subcommand() {
	Some(("build", _)) => build(&whileyhome),
	Some(("clean", _)) => clean(&whileyhome),
	Some(("init", _)) => init(&whileyhome),	
	_ => unreachable!()
    }
}
