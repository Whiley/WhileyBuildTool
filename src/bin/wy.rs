use clap::{arg, Command};
use std::error::Error;
use log::LevelFilter;
use whiley::command::{build,clean,init,install};
use whiley::{init_logging,init_whileyhome};

fn main() -> Result<(),Box<dyn Error>> {
    // Parse command-line arguments
    let matches = Command::new("wy")
	.about("Whiley Build Tool")
	.version("0.6.0")
        .subcommand_required(true)
	.arg(arg!(--verbose "Show verbose output"))
	.subcommand(
	    Command::new("build").about("Build local package(s)"))
	.subcommand(
	    Command::new("clean").about("Remove all generated (binary) files"))
	.subcommand(
	    Command::new("init").about("Create a new Whiley package in an existing directory"))
	.subcommand(
	    Command::new("install").about("Install package in local repository"))
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
    let ok = match matches.subcommand() {
	Some(("build", _)) => build(&whileyhome),
	Some(("clean", _)) => clean(&whileyhome),
	Some(("init", _)) => init(&whileyhome),
	Some(("install", _)) => install(&whileyhome),
	_ => unreachable!()
    }?;
    // Determine appropriate exit code
    let exitcode = if ok { 0 } else { 1 };
    // Done
    std::process::exit(exitcode);
}
