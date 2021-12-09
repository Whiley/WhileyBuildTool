use clap::{App, AppSettings};

pub fn main() {
    // Parse command-line arguments
    let matches = App::new("wy")
	.about("Whiley's Build Tool and Package Manager")
	.version("0.6.0")
	.setting(AppSettings::SubcommandRequiredElseHelp)
	.subcommand(
	    App::new("build").about("Build local package(s)"))
	.subcommand(
	    App::new("clean").about("Remove all generated build artifact(s)"))	
	.get_matches();
    // Dispatch on outcome
    match matches.subcommand() {
	("build", Some(_)) => {
	    println!("Build not implemented yet!");
	}
	("clean", Some(_)) => {
	    println!("Clean not implemented yet!");
	}
	_ => unreachable!()
    }
}
